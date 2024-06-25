// shard_compressor.c
#include <lz4.h>
#include <lz4hc.h>

void compress_shard_data(char* data, int data_len, char* compressed_data) {
    // Advanced shard data compression using LZ4
    LZ4_stream_t* lz4_stream = LZ4_createStream();
    LZ4_compress_HC(lz4_stream, data, data_len, compressed_data, LZ4_COMPRESSBOUND(data_len), LZ4HC_CLEVEL_OPT_DEFAULT);
    LZ4_freeStream(lz4_stream);
}

void decompress_shard_data(char* compressed_data, int compressed_len, char* decompressed_data) {
    // Advanced shard data decompression using LZ4
    LZ4_stream_t* lz4_stream = LZ4_createStream();
    LZ4_decompress_safe(compressed_data, compressed_len, decompressed_data, LZ4_COMPRESSBOUND(compressed_len));
    LZ4_freeStream(lz4_stream);
}
