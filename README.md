# Basics

The repo specifies the **CacheSim** trait as the interface required for all cache simulators, and the **ObjIdTraits** trait wrapping trait bounds for the type of objects stored in the cache. Example implementation can be found (later) in Oz cache.



# Usage
To import directly from git, under Cargo.toml [dependencies], add

        abstract_cache = {git = "https://github.com/Roc-Locality/abstract_cache"}


# Change to associated type (9/5/24)

The implementation has changed from using the ObjId generic to an associated type with the same trait bounds. The purpose is to make the CacheSim trait object safe. For previously implemented simulators, a two-line change will be necessary.

The old implementation

        impl <...> CacheSim<Foo> for YourSim<...> {
        ...

becomes

        impl <...> CacheSim for YourSim<...> {
                type ObjId = Foo;
        ...

**Foo** can be a type or generic, and still needs to satisfy the trait bounds of ObjIdTraits. This includes built-in integer types, string types, or custom-defined types implementing ObjIdTraits. 
