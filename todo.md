# rx / redis : todo lists

## Step 1

### Test Applicaition

A test application that continuously writes new and modified data to the rxkv / redis server via the client.

* enhance redis client/server to support
  * ~~length of current db~~
  * return all keys
  * read static data on server startup
* use fake-rs + domain keys to generate new struct data
    * ~~add domain_keys dep: `cargo add --path ../domain-keys`~~
    * ~~add fake-rs dep: `cargo add --path ../fake-rs/fake`~~
    * ~~modify examples/pub to create a series of rt keys to publish~~
    * implement person struct in models (remove repo?)
* create write thread with random intervals timed for about 2 to 5 writes per second
* create read thread with random intervals timed for about 20 to 30 reads per second
* use pub/sub to share data with all active application instances
* use channels to pull data from subscribers and store in local key vector
* add logging pub/sub channel

## Step 2

* add daemon capabilities to db-server
* ~~test remote access to server (piedmont)~~

## Misc

* strip out all telemetry and deps (after review)
* add rolling file based logging to server
* read config from file to set host/port 
* unit and doc tests

###### darryl.west | 2022-10-10a

