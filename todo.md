# rx / redis : todo lists

## Step 1

### Test Applicaition

A test application that continuously writes new and modified data to the rxkv / redis server via the client.

* use fake-rs + domain keys to generate new struct data
    * ~~add domain_keys dep: `cargo add --path ../domain-keys`~~
    * ~~add fake-rs dep: `cargo add --path ../fake-rs/fake`~~
    * ~~modify examples/pub to create a series of rt keys to publish~~
    * implement person struct in models (remove repo?)
* create write thread with random intervals timed for about 2 to 5 writes per second
* create read thread with random intervals timed for about 20 to 30 reads per second
* enhance redis client/server to support
  * return all keys
  * length of current db
* use pub/sub to share data with all active application instances
* use channels to pull data from subscribers and store in local key vector

## Step 2

* add daemon capabilities to db-server
* test remote access to server (piedmont)

###### darryl.west | 2022-10-10

