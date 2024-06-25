// shard_data_encryption.cpp
#include <iostream>
#include <helib/helib.h>

int main() {
  // Initialize the homomorphic encryption library
  helib::Context context = helib::ContextBuilder<helib::BGV>()
   .m(1024)
   .p(2)
   .r(3)
   .d(1)
   .c(2)
   .build();

  // Encrypt shard data using homomorphic encryption
  helib::SecKey secretKey(context);
  helib::PubKey publicKey = secretKey;
  helib::Ctxt ctxt(publicKey);
  std::string plaintext = "Shard data";
  ctxt.encrypt(publicKey, plaintext);

  // Decrypt shard data using homomorphic encryption
  std::string decryptedText;
  ctxt.decrypt(secretKey, decryptedText);
  std::cout <<"Decrypted text: " << decryptedText << std::endl;

  return 0;
}
