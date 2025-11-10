# Tips and Tricks

1. **Keep It Simple**
    - QuokkaSim allows you to very easily refactor from using a ``f64`` as a resource, to using a ``[f64; N]`` as a resource. However, if your modelling only depends on totals (but not for example, the makeup of that total), it is always easier to stay with a ``f64`` resource. Adding complexity always increases the amount of time you'll need to build, fix and run the model - so when adding complexity, this should be for good reason.
2. **Model elements need not map directly to real-life elements**
    - In reality there may be processes where no buffer exists between the two, which seems to conflict with QuokkaSim's convention of not connecting processes directly. There's nothing stopping you from implementing such a connection, however in almost all desired use cases, equivalent behaviour can be performed with a 'virtual' stock in between. For example, if Process A feeds C, a virtual Stock B can be placed in between, and Process A may only continue with further inputs once Stock B is empty.
3. **Don't be afraid to create your own Processes**
    - QuokkaSim provides some sensible default processes to accelerate the model building progress, but there are many legitimate behaviours a process may want to have. For this reason, QuokkaSim provides seamless support for defining your own structs.
