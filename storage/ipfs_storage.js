// storage/ipfs_storage.js
const ipfs = require('ipfs-api')();

class IPFSStorage {
    async storeData(data) {
        const file = new ipfs.types.Buffer(data);
        const result = await ipfs.add(file);
        return result[0].hash;
    }

    async retrieveData(hash) {
        const file = await ipfs.cat(hash);
        return file.toString();
    }
}
