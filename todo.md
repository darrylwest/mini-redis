# rx / redis : todo lists

## Step 1

### Test Applicaition

A test application that continuously writes new and modified data to the rxkv / redis server via the client.

* enhance redis client/server to support
  * ~~length of current db~~
  * _read static data on server startup (partial insert of mock data in place)._
  * return all keys with offset and limit
  * determine how to write a json string to Bytes or BytesMut for Entry.data
  * figure out what the id in Entry is for and see if it can (should) be removed
* use fake-rs + domain keys to generate new struct data
    * ~~add domain_keys dep: `cargo add --path ../domain-keys`~~
    * ~~add fake-rs dep: `cargo add --path ../fake-rs/fake`~~
    * ~~modify examples/pub to create a series of rt keys to publish~~
    * implement person struct in models (remove repo?)

## Step 2

* create write thread with random intervals timed for about 2 to 5 writes per second
* create read thread with random intervals timed for about 20 to 30 reads per second
* use pub/sub to share data with all active application instances
* use channels to pull data from subscribers and store in local key vector
* add logging pub/sub channel

## Step 3

* add daemon capabilities to db-server
* ~~test remote access to server (piedmont)~~

## Misc

* ~~replace dbsize with count~~ and count_by(predicate)
* replace json transport with message pack
* strip out all telemetry and deps (after review)
* add rolling file based logging to server
* read config from file to set host/port 
* unit and doc tests
* add github/actions to test merge to main

###### darryl.west | 2022-10-10a

