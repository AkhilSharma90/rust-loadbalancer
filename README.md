# rust-loadbalancer
A simple round robin load balancer in rust
Here, we define a `LoadBalancer` struct that stores a list of server URLs and a mutex-protected index that keeps track of the next server to use. The `new()` method creates a new instance of LoadBalancer, and the `next_server()` method returns the next server URL in the list using a round-robin algorithm.

In the `main()` function, we create a vector of server URLs and wrap it in an Arc to allow multiple threads to access it. We then spawn 10 threads, each of which sends 10 requests to the load balancer by calling the `next_server()` method. Finally, we wait for 1 second to allow all the threads to finish.

