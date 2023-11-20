//#include "common.h"
#include "llama.h"

#include <iostream>
#include <vector>

int main(int argc, char** argv) {
    llama_model_params model_params = llama_model_default_params();
    std::string model_path = "models/llama-2-7b-chat.Q4_0.gguf";
    std::cout << "llama.cpp example using model: " << model_path << std::endl;

    std::string prompt = "Who is Austin Powers?";
    std::cout << "prompt: " << prompt << std::endl;

    bool numa = false;
    llama_backend_init(numa);

    llama_model* model = llama_load_model_from_file(model_path.c_str(), model_params);
    if (model == NULL) {
        fprintf(stderr , "%s: error: failed to to load model %s\n" , __func__, model_path.c_str());
        return 1;
    }

    llama_context_params ctx_params = llama_context_default_params();
    ctx_params.seed  = 1234;
    ctx_params.n_ctx = 2048;
    //ctx_params.n_threads = params.n_threads;
    //ctx_params.n_threads_batch = params.n_threads_batch == -1 ? params.n_threads : params.n_threads_batch;

    llama_context * ctx = llama_new_context_with_model(model, ctx_params);
    if (ctx == NULL) {
        fprintf(stderr , "%s: error: failed to create the llama_context\n" , __func__);
        return 1;
    }

    const int add_bos_token = llama_add_bos_token(model);
    const bool add_bos  = add_bos_token != -1 ? bool(add_bos_token) :
        (llama_vocab_type(model) == LLAMA_VOCAB_TYPE_SPM); // SPM = SentencePiece Model
    int n_tokens = prompt.length() + add_bos;
    std::vector<llama_token> input_tokens(n_tokens);
    n_tokens = llama_tokenize(model,
                              prompt.data(),
                              prompt.length(),
                              input_tokens.data(),
                              input_tokens.size(),
                              true,
                              false);

    std::cout << "n_tokens: " << n_tokens << std::endl;

    for (int i = 0; i < n_tokens; i++) {
        std::cout << "input_tokens[" << i << "]: " << input_tokens[i] << std::endl;
    //for (auto token : input_tokens) {
        std::vector<char> result(8, 0);
        int token_len = result.size();
        int n_tokens = llama_token_to_piece(model, input_tokens[i], result.data(), token_len);
        // llama_token_to_piece will return the negative length of the token if
        // it is longer that the passed in result.length. If that is the case
        // then we need to resize the result vector to the length of the token
        // and call llama_token_to_piece again.
        if (n_tokens < 0) {
            result.resize(-n_tokens);
            int new_len = llama_token_to_piece(model, input_tokens[i], result.data(), result.size());
            std::cout << "new_len: " << new_len << " vs token_len: " << token_len << std::endl;
        } else {
            result.resize(n_tokens);
        }
        std::string token_str = std::string(result.data(), result.size());
        std::cout << "token_str: " << token_str.c_str() << std::endl;
    }

    llama_batch batch = llama_batch_init(512, 0, 1);
    //batch.n_tokens = input_tokens.size();
    //std::cout << "batch.n_tokens: " << batch.n_tokens << std::endl;

    // So what this llmm_batch is used for is similar to the concept of context
    // we talked about in ../../notes/llm.md#context_size. Below we are adding
    // the input query tokens to this batch/context. So it will initially just
    // contain the tokens for our query. But after running the inference, we
    // will append the next token to the batch and run the inference again and
    // then run the inference again to predict the next token, now with more
    // context (the previous token). Hope this makes sense but looking at the
    // diagram in the notes might help.
    bool logits = false;
    const std::vector<llama_seq_id>& seq_ids = { 0 };

    std::cout << "batch.n_tokens: " << batch.n_tokens << std::endl;
    for (size_t pos = 0; pos < input_tokens.size(); pos++) {
        //llama_batch_add(batch, input_tokens[pos], pos, seq_ids, logits);
        batch.token[batch.n_tokens] = input_tokens[pos];
        batch.pos[batch.n_tokens] = pos,
        batch.n_seq_id[batch.n_tokens] = seq_ids.size();
        batch.logits[batch.n_tokens] = logits;
        batch.n_tokens++;
    }

    // Instruct llama to generate the logits for the last token
    batch.logits[batch.n_tokens - 1] = true;

    // Now we run the inference on the batch. This will populate the logits
    // for the last token in the batch.
    if (llama_decode(ctx, batch) != 0) {
        fprintf(stderr, "llama_decode() failed\n");
        return 1;
    }

    // This is the total number of tokens that we will generate, which recall
    // includes our query tokens (they are all in the llm_batch).
    const int n_gen_tokens = 32;

    int n_cur = batch.n_tokens;
    int n_decode = 0;
    //std::cout << "n_vocab: " << n_vocab << std::endl;
    std::cout << "LLM response:" << std::endl;
    while (n_cur <= n_gen_tokens) {
        {
            // logits are stored in the last token of the batch and are
            // logits are the raw unnormalized predictions
            int n_vocab = llama_n_vocab(model);
            float* logits = llama_get_logits_ith(ctx, batch.n_tokens - 1);
            // logits will be an array of length 32000 because it will be resized
            // by the above call to llama_decode.

            std::vector<llama_token_data> candidates;
            candidates.reserve(n_vocab);

            // The following is populating the candidates vector with the
            // logit for each token in the vocabulary (32000).
            for (llama_token token_id = 0; token_id < n_vocab; token_id++) {
                candidates.emplace_back(llama_token_data{ token_id, logits[token_id], 0.0f });
            }
            // Here we are creating an unsorted array of token data from the vector.
            bool sorted = false;
            llama_token_data_array candidates_p = { candidates.data(), candidates.size(), sorted };

            // Find the token with the highest raw score (logit) and return it.
            const llama_token highest_logit = llama_sample_token_greedy(ctx, &candidates_p);

            // is it an end of stream?
            if (highest_logit == llama_token_eos(model) || n_cur == n_gen_tokens) {
                fprintf(stdout, "\n");
                fflush(stdout);
                break;
            }

            // Next we get the string value for the token. This is called a piece
            // which I think comes from SentencePiece, and a token would be
            // one such piece (something like that).
            //fprintf(stdout, "%s", llama_token_to_piece(ctx, highest_logit).c_str());
            //fflush(stdout);

            // push this new token for next evaluation
            llama_batch_free(batch);
            //llama_batch_init(batch, highest_logit, n_cur, { 0 }, true);
            batch.token[batch.n_tokens] = highest_logit;
            batch.pos[batch.n_tokens] = n_cur,
            batch.n_seq_id[batch.n_tokens] = { 0 };
            batch.logits[batch.n_tokens] = true; // logits
            batch.n_tokens++;
            n_decode += 1;
        }

        n_cur += 1;

        // With the new token added to the batch, we can now predict the
        // next token with the logit from above and repeat the process.
        if (llama_decode(ctx, batch)) {
            fprintf(stderr, "%s : failed to eval, return code %d\n", __func__, 1);
            return 1;
        }
    }
    llama_batch_free(batch);
    llama_free(ctx);
    llama_free_model(model);
    llama_backend_free();

    return 0;
}
