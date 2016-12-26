# sturdy-umbrella
 Trying to do fun things with Rust. Also, what's Rust?

I suggest that we run all of code in a Linux VM just so we're running the same system.
Let's at least each stick with some flavor of Ubuntu (I tend to use Xubntu a lot)

Rust resources: (Feel free to add to this list)
* https://doc.rust-lang.org/book/
* http://rustbyexample.com/

In general, I would like to follow (a few) best practices for software engineering
for the sake of our sanity.
Let's at a least:
* plan out our system before we create it (can be a rough plan)
* WRITE TESTS FOR OUR CODE (Rust has built-in support for unit tests :D)
GOT Xubuntu WUBALUBADUBDUB

So assuming we're doing the NN thing, I found a NN crate (Rust package) somebody made already:
https://github.com/jackm321/RustNN
It looks pretty decent. Anyways it's there if we ever want a reference.

The next step (besides from learning Rust - BTW, I'm going through the book tutorial thing and 
Rust ownership, i.e. Rust pointers are cray. Super cool, but definitely a bit of a mind bender)
is to decide what types of NNs our crate (I assume we're making a crate) is going to handle.
The one above only does feed forward networks with back propogation which is the standard thing.
Based on me spending a minute looking at his code, I don't see any options for how the network is 
connected (fully connected, partially, etc) or for different threshold/acticvation functions 
(Sigmoid vs. Tau - he only does sigmoid). I'm perfectly happy with just implementing the a fully
connected feed forward network, but we should plan out what we'll support before we start coding.
Oh, he also has a useful function to dump the NN out as a JSON file and read in said JSON as a NN.
He also has an error rate function but I can't figure out what the hell it's calculating.

Anyways, I'm gonna make a file for us to map out hte functionality we're gonna implement

Also, some NN resources for our reference:
https://en.wikipedia.org/wiki/Artificial_neural_network#Models
http://pages.cs.wisc.edu/~bolo/shipyard/neural/local.html
http://www.cse.msu.edu/~cse802/notes/ArtificialNeuralNetworks.pdf
http://ufldl.stanford.edu/wiki/index.php/Neural_Networks
http://ufldl.stanford.edu/wiki/index.php/Backpropagation_Algorithm
http://colah.github.io/posts/2015-08-Understanding-LSTMs/
