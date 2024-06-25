// shard_master/storage/optane_storage.cpp
#include <iostream>
#include <fstream>
#include <string>
#include <vector>

#include <intel/optane.h>

class OptaneStorage {
public:
    OptaneStorage(const std::string& path) : path_(path) {}

    void write(const std::vector<char>& data) {
        // Use Intel Optane to write data to storage
        optane_write(path_.c_str(), data.data(), data.size());
    }

    std::vector<char> read(size_t size) {
        // Use Intel Optane to read data from storage
        std::vector<char> data(size);
        optane_read(path_.c_str(), data.data(), size);
        return data;
    }

private:
    std::string path_;
};
