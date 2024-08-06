At the boundaries, the inputs and outputs to the fundcast core library will be the library-provided `Money` type, but the internals should probably just use integers? Or maybe not, because we want to be explicit about what operations are allowed (dollars * dollars doesn't really make any sense).

Should the currency be part of the money object though? Then we have to deal with checking whether two added moneys are of the same currency, and probably panic if not. The currency should probably be defined at the wallet-level (not sure whether to use wallet or vault), but still need to decide if the Money object should have a way to represent its own currency. Why don't we exclude currency from the money object for now and see if we can make that work, I think we can by having printing methods be on the Wallet instance, rather than the Money instance.

- [ ] TODO: Decide if we still want to use the **currencies** crate, otherwise remove it from the dependencies.