#include "kiwi.h"

namespace cakectf {

#ifndef INCLUDE_CAKECTF_H
#define INCLUDE_CAKECTF_H

class BinarySchema {
public:
  bool parse(kiwi::ByteBuffer &bb);
  const kiwi::BinarySchema &underlyingSchema() const { return _schema; }
  bool skipEncryptionKeyField(kiwi::ByteBuffer &bb, uint32_t id) const;

private:
  kiwi::BinarySchema _schema;
  uint32_t _indexEncryptionKey = 0;
};

class EncryptionKey;

class EncryptionKey {
public:
  EncryptionKey() { (void)_flags; }

  uint32_t *magic();
  const uint32_t *magic() const;
  void set_magic(const uint32_t &value);

  kiwi::Array<uint8_t> *key();
  const kiwi::Array<uint8_t> *key() const;
  kiwi::Array<uint8_t> &set_key(kiwi::MemoryPool &pool, uint32_t count);

  bool encode(kiwi::ByteBuffer &bb);
  bool decode(kiwi::ByteBuffer &bb, kiwi::MemoryPool &pool, const BinarySchema *schema = nullptr);

private:
  uint32_t _flags[1] = {};
  kiwi::Array<uint8_t> _data_key = {};
  uint32_t _data_magic = {};
};

#endif
#ifdef IMPLEMENT_SCHEMA_H

bool BinarySchema::parse(kiwi::ByteBuffer &bb) {
  if (!_schema.parse(bb)) return false;
  _schema.findDefinition("EncryptionKey", _indexEncryptionKey);
  return true;
}

bool BinarySchema::skipEncryptionKeyField(kiwi::ByteBuffer &bb, uint32_t id) const {
  return _schema.skipField(bb, _indexEncryptionKey, id);
}

uint32_t *EncryptionKey::magic() {
  return _flags[0] & 1 ? &_data_magic : nullptr;
}

const uint32_t *EncryptionKey::magic() const {
  return _flags[0] & 1 ? &_data_magic : nullptr;
}

void EncryptionKey::set_magic(const uint32_t &value) {
  _flags[0] |= 1; _data_magic = value;
}

kiwi::Array<uint8_t> *EncryptionKey::key() {
  return _flags[0] & 2 ? &_data_key : nullptr;
}

const kiwi::Array<uint8_t> *EncryptionKey::key() const {
  return _flags[0] & 2 ? &_data_key : nullptr;
}

kiwi::Array<uint8_t> &EncryptionKey::set_key(kiwi::MemoryPool &pool, uint32_t count) {
  _flags[0] |= 2; return _data_key = pool.array<uint8_t>(count);
}

bool EncryptionKey::encode(kiwi::ByteBuffer &_bb) {
  if (magic() != nullptr) {
    _bb.writeVarUint(1);
    _bb.writeVarUint(_data_magic);
  }
  if (key() != nullptr) {
    _bb.writeVarUint(2);
    _bb.writeVarUint(_data_key.size());
    for (uint8_t &_it : _data_key) _bb.writeByte(_it);
  }
  _bb.writeVarUint(0);
  return true;
}

bool EncryptionKey::decode(kiwi::ByteBuffer &_bb, kiwi::MemoryPool &_pool, const BinarySchema *_schema) {
  uint32_t _count;
  while (true) {
    uint32_t _type;
    if (!_bb.readVarUint(_type)) return false;
    switch (_type) {
      case 0:
        return true;
      case 1: {
        if (!_bb.readVarUint(_data_magic)) return false;
        set_magic(_data_magic);
        break;
      }
      case 2: {
        if (!_bb.readVarUint(_count)) return false;
        for (uint8_t &_it : set_key(_pool, _count)) if (!_bb.readByte(_it)) return false;
        break;
      }
      default: {
        if (!_schema || !_schema->skipEncryptionKeyField(_bb, _type)) return false;
        break;
      }
    }
  }
}

#endif

}
