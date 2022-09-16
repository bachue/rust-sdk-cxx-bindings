#include "includes/qiniu_sdk_ffi.h"
#include <memory>
#include <iostream>

static inline uint8_t get_errbit(std::istream *stream);

namespace qiniu_sdk_ffi
{
    namespace rust
    {
        size_t SeekableReader::read(uint8_t *data, size_t size, uint8_t *errbit) const
        {
            this->stream->read(static_cast<char *>(static_cast<void *>(data)), size);
            *errbit = get_errbit(this->stream);
            this->stream->clear();
            return this->stream->gcount();
        }

        uint64_t SeekableReader::seek(int64_t off, uint8_t pos, uint8_t *errbit) const
        {
            std::ios_base::seekdir dir;
            switch (pos)
            {
            case 0:
                dir = std::ios_base::beg;
                break;
            case 1:
                dir = std::ios_base::cur;
                break;
            case 2:
                dir = std::ios_base::end;
                break;
            default:
                throw std::runtime_error("invalid pos");
            }
            this->stream->seekg(off, dir);
            *errbit = get_errbit(this->stream);
            this->stream->clear();
            return this->stream->tellg();
        }

        std::unique_ptr<SeekableReader> new_seekable_reader(void *ptr)
        {
            std::istream *stream = static_cast<std::istream *>(ptr);
            return std::unique_ptr<SeekableReader>(new SeekableReader(stream));
        }
    }
}

static inline uint8_t get_errbit(std::istream *stream)
{
    uint8_t errbit = 0;
    if (stream->bad())
    {
        errbit |= 1;
    }
    else if (stream->fail() && !stream->eof())
    {
        errbit |= 2;
    }
    return errbit;
}
