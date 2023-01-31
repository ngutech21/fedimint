# Storagemint
The storage-module adds the capability of storing files for users. It can be used to store images, videos, documents, etc. Files can either be stored and retrieved by a command line client (cli) or by using the storage-server. The storage-server crate provides a Rest-API and a simple web interface to access the stored data.

Possible usecases:
* Store torrent trackerfiles, profilepics, memes and other images that are publicly available 
* Backup important documents like contracts, invoices, sharded private keys, etc. that are only accessible by the owner
* Let the users pay using lightning e.g. pay per kb of storage or pay per download


# Implementation
The client-module generates a UUID that is used as a key to identify the file. The content of the file is BASE64 encoded and stored as value in the storage-module. In the future metadata like tags could be added to find publicly available data. 



# Installation
Clone the repo and run the tmuxinator script to start the federation:
```
git clone https://github.com/ngutech21/fedimint.git
cd  fedimint
git checkout feat-fedimint-storage
nix develop
./scripts/tmuxinator.sh
```


# Usage
## CLI
Store a file in the federation by using the cli
```
fedimint-cli store-data dummy.txt
```
returns the key (UUID) of the stored file 
{
  "key": "17fc82aa-d3f1-478c-947d-aa2736578ec2"
}


retrieve a previously stored file by using the cli. First argument is the key (UUID) and then the output file
```
fedimint-cli get-data 17fc82aa-d3f1-478c-947d-aa2736578ec2 output.txt
```


## Storage-Server
The storage-server is a http server that can be used to store and retrieve files. It is a simple wrapper around the client-module and provides a Rest-API and a simple web interface to access the storage-module. To upload a new file click on browse and select an image. After the image is selected click on upload image and the image will be added to the page. Every image on the page has a download button to download the file and a permalink to share the file. The webui only supports storing images at the moment, but the Rest-API and the cli can be used to store any kind of file.


### Implementation details:
The web interface uses vuejs and is served by an axum webserver. In the future the frontend will use the client-module compiled as wasm, so the frontend can communicate directly with the storage-module. 

```
cd storage-server
cargo run
```

open browser and go to http://127.0.0.1:8080

![screenshot of the storageming webapp](Storagemint-startpage.jpg)



