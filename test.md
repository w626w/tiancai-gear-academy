##### 库文件里报错（最后），在github的工作流里面会一直报错，只能在我本地库里面修改，修改之后可以正常执行工作流

在库文件里报这个错误：

```


error[E0308]: arguments to this function are incorrect
   --> /home/runner/.cargo/git/checkouts/ring-vrf-fa8519651343a7ee/4b09416/bandersnatch_vrfs/src/ring.rs:85:9
    |
85  |         ring::index(self.pcs_params.clone(), &self.piop_params, pks).0
    |         ^^^^^^^^^^^
    |
note: expected `&_`, found `URS<Bls12<Config>>`
   --> /home/runner/.cargo/git/checkouts/ring-vrf-fa8519651343a7ee/4b09416/bandersnatch_vrfs/src/ring.rs:85:21
    |
85  |         ring::index(self.pcs_params.clone(), &self.piop_params, pks).0
    |                     ^^^^^^^^^^^^^^^^^^^^^^^
    = note: expected reference `&_`
                  found struct `URS<Bls12<ark_bls12_381::Config>>`
note: expected `&[Affine<BandersnatchConfig>]`, found `Vec<Affine<BandersnatchConfig>>`
   --> /home/runner/.cargo/git/checkouts/ring-vrf-fa8519651343a7ee/4b09416/bandersnatch_vrfs/src/ring.rs:85:65
    |
85  |         ring::index(self.pcs_params.clone(), &self.piop_params, pks).0
    |                                                                 ^^^
    = note: expected reference `&[ark_ec::short_weierstrass::Affine<BandersnatchConfig>]`
                  found struct `Vec<ark_ec::short_weierstrass::Affine<BandersnatchConfig>>`
note: function defined here
   --> /home/runner/.cargo/git/checkouts/ring-proof-e9e49c3c86c409a2/665f5f5/ring/src/piop/mod.rs:222:8
    |
222 | pub fn index<F: PrimeField, CS: PCS<F>, Curve: SWCurveConfig<BaseField=F>>(
    |        ^^^^^
help: consider borrowing here
    |
85  |         ring::index(&self.pcs_params.clone(), &self.piop_params, pks).0
    |                     +
help: consider borrowing here
    |
85  |         ring::index(self.pcs_params.clone(), &self.piop_params, &pks).0
    |                                                                 +

   Compiling enumset_derive v0.10.0
error[E0308]: arguments to this function are incorrect
   --> /home/runner/.cargo/git/checkouts/ring-vrf-fa8519651343a7ee/4b09416/bandersnatch_vrfs/src/ring.rs:89:9
    |
89  |         ring::index(self.pcs_params.clone(), &self.piop_params, pks).1
    |         ^^^^^^^^^^^
    |
note: expected `&_`, found `URS<Bls12<Config>>`
   --> /home/runner/.cargo/git/checkouts/ring-vrf-fa8519651343a7ee/4b09416/bandersnatch_vrfs/src/ring.rs:89:21
    |
89  |         ring::index(self.pcs_params.clone(), &self.piop_params, pks).1
    |                     ^^^^^^^^^^^^^^^^^^^^^^^
    = note: expected reference `&_`
                  found struct `URS<Bls12<ark_bls12_381::Config>>`
note: expected `&[Affine<BandersnatchConfig>]`, found `Vec<Affine<BandersnatchConfig>>`
   --> /home/runner/.cargo/git/checkouts/ring-vrf-fa8519651343a7ee/4b09416/bandersnatch_vrfs/src/ring.rs:89:65
    |
89  |         ring::index(self.pcs_params.clone(), &self.piop_params, pks).1
    |                                                                 ^^^
    = note: expected reference `&[ark_ec::short_weierstrass::Affine<BandersnatchConfig>]`
                  found struct `Vec<ark_ec::short_weierstrass::Affine<BandersnatchConfig>>`
note: function defined here
   --> /home/runner/.cargo/git/checkouts/ring-proof-e9e49c3c86c409a2/665f5f5/ring/src/piop/mod.rs:222:8
    |
222 | pub fn index<F: PrimeField, CS: PCS<F>, Curve: SWCurveConfig<BaseField=F>>(
    |        ^^^^^
help: consider borrowing here
    |
89  |         ring::index(&self.pcs_params.clone(), &self.piop_params, pks).1
    |                     +
help: consider borrowing here
    |
89  |         ring::index(self.pcs_params.clone(), &self.piop_params, &pks).1
    |                                                                 +

For more information about this error, try `rustc --explain E0308`.
error: could not compile `bandersnatch_vrfs` (lib) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Error: Process completed with exit code 101.
```

