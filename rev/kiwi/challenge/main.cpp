#define IMPLEMENT_KIWI_H
#define IMPLEMENT_SCHEMA_H

#include <boost/algorithm/hex.hpp>
#include <fstream>
#include <iostream>
#include <vector>
#include <iomanip>
#include "cakectf.hpp"

int readFlag(std::string &flag) {
  std::ifstream ifs("./flag.txt");

  if (ifs.fail())
    return 1;

  getline(ifs, flag);
  return 0;
}

int readKey(kiwi::ByteBuffer &message) {
  std::vector<uint8_t> bytes_key;
  std::string hex_key;

  std::cout << "Enter key: ";
  std::cin >> hex_key;
  if (!std::cin.good())
    return 1;

  try {
    boost::algorithm::unhex(hex_key, std::back_inserter(bytes_key));
  } catch(...) {
    return 1;
  }
  for (uint8_t c: bytes_key) {
    message.writeByte(c);
  }

  return 0;
}

int checkMessage(const cakectf::EncryptionKey &enckey) {
  if (enckey.magic() == nullptr)
    return 1;
  if (*enckey.magic() != 0xCAFEC4F3)
    return 1;
  if (enckey.key() == nullptr)
    return 1;
  if (enckey.key()->size() < 8)
    return 1;

  return 0;
}

std::vector<uint8_t> encryptFlag(const std::string &flag,
                                 const kiwi::Array<uint8_t> &key) {
  std::vector<uint8_t> enc_flag;
  enc_flag.reserve(flag.size());

  for (size_t i = 0; i < flag.size(); i++) {
    enc_flag.emplace_back(flag[i] ^ key.data()[i % key.size()] ^ 0xff ^ i);
  }

  return enc_flag;
}

int main() {
  std::string flag;
  kiwi::ByteBuffer message;
  kiwi::MemoryPool pool;
  cakectf::EncryptionKey enckey;

  std::setbuf(stdin, NULL);
  std::setbuf(stdout, NULL);
  std::setbuf(stderr, NULL);

  if (readFlag(flag)) {
    std::cerr << "[-] Failed to open flag." << std::endl;
    return 1;
  }
  if (readKey(message)) {
    std::cerr << "[-] Failed to read key." << std::endl;
    return 1;
  }

  if (!enckey.decode(message, pool)) {
    std::cerr << "[-] Failed to decode key." << std::endl;
    return 1;
  }
  if (checkMessage(enckey)) {
    std::cerr << "[-] Invalid key." << std::endl;
    return 1;
  }

  std::vector<uint8_t> enc_flag = encryptFlag(flag, *enckey.key());
  std::cout << "Encrypted flag: ";
  for (uint8_t c: enc_flag) {
    std::cout << std::setfill('0') << std::setw(2) << std::hex << (uint32_t)c;
  }
  std::cout << std::endl;

  return 0;
}
