#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::{fs::File, io::{self, Write}};
use bitcode::{Decode, Encode};
const NUM_RESOURCES: usize = 2048;
const CLIENT_SIZE: usize = 5_000_000;
struct CustomerServers {
    servers: Vec<String>,
    customers: Vec<String>,
}
const _: () = {
    impl bitcode::__private::Encode for CustomerServers {
        type Encoder = CustomerServersEncoder;
    }
    #[allow(non_snake_case)]
    pub struct CustomerServersEncoder {
        servers: <Vec<String> as bitcode::__private::Encode>::Encoder,
        customers: <Vec<String> as bitcode::__private::Encode>::Encoder,
    }
    impl std::default::Default for CustomerServersEncoder {
        fn default() -> Self {
            Self {
                servers: Default::default(),
                customers: Default::default(),
            }
        }
    }
    impl bitcode::__private::Encoder<CustomerServers> for CustomerServersEncoder {
        #[inline(always)]
        fn encode(&mut self, v: &CustomerServers) {
            #[allow(unused_imports)]
            use bitcode::__private::Buffer as _;
            let CustomerServers { servers, customers } = v;
            self.servers.encode(servers);
            self.customers.encode(customers);
        }
        fn encode_vectored<'__v>(
            &mut self,
            i: impl Iterator<Item = &'__v CustomerServers> + Clone,
        )
        where
            CustomerServers: '__v,
        {
            #[allow(unused_imports)]
            use bitcode::__private::Buffer as _;
            self.servers
                .encode_vectored(
                    i
                        .clone()
                        .map(|me| {
                            let servers = &me.servers;
                            servers
                        }),
                );
            self.customers
                .encode_vectored(
                    i
                        .clone()
                        .map(|me| {
                            let customers = &me.customers;
                            customers
                        }),
                );
        }
    }
    impl bitcode::__private::Buffer for CustomerServersEncoder {
        fn collect_into(&mut self, out: &mut Vec<u8>) {
            self.servers.collect_into(out);
            self.customers.collect_into(out);
        }
        fn reserve(&mut self, __additional: std::num::NonZeroUsize) {
            self.servers.reserve(__additional);
            self.customers.reserve(__additional);
        }
    }
};
const _: () = {
    impl<'__de> bitcode::__private::Decode<'__de> for CustomerServers
    where
        '__de:,
    {
        type Decoder = CustomerServersDecoder<'__de>;
    }
    #[allow(non_snake_case)]
    pub struct CustomerServersDecoder<'__de> {
        servers: <Vec<String> as bitcode::__private::Decode<'__de>>::Decoder,
        customers: <Vec<String> as bitcode::__private::Decode<'__de>>::Decoder,
    }
    impl<'__de> std::default::Default for CustomerServersDecoder<'__de> {
        fn default() -> Self {
            Self {
                servers: Default::default(),
                customers: Default::default(),
            }
        }
    }
    impl<'__de> bitcode::__private::View<'__de> for CustomerServersDecoder<'__de> {
        fn populate(
            &mut self,
            input: &mut &'__de [u8],
            __length: usize,
        ) -> bitcode::__private::Result<()> {
            self.servers.populate(input, __length)?;
            self.customers.populate(input, __length)?;
            Ok(())
        }
    }
    impl<'__de> bitcode::__private::Decoder<'__de, CustomerServers>
    for CustomerServersDecoder<'__de>
    where
        '__de:,
    {
        #[inline(always)]
        fn decode_in_place(&mut self, out: &mut std::mem::MaybeUninit<CustomerServers>) {
            self.servers
                .decode_in_place(unsafe {
                    &mut *(&raw mut (*out.as_mut_ptr()).servers
                        as *mut std::mem::MaybeUninit<Vec<String>>)
                });
            self.customers
                .decode_in_place(unsafe {
                    &mut *(&raw mut (*out.as_mut_ptr()).customers
                        as *mut std::mem::MaybeUninit<Vec<String>>)
                });
        }
    }
};
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for CustomerServers {}
#[automatically_derived]
impl ::core::cmp::PartialEq for CustomerServers {
    #[inline]
    fn eq(&self, other: &CustomerServers) -> bool {
        self.servers == other.servers && self.customers == other.customers
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CustomerServers {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "CustomerServers",
            "servers",
            &self.servers,
            "customers",
            &&self.customers,
        )
    }
}
fn main() -> io::Result<()> {
    let cs = CustomerServers {
        servers: (1..=NUM_RESOURCES)
            .map(|i| {
                let res = ::alloc::fmt::format(format_args!("server{0}.theykk.net", i));
                res
            })
            .collect(),
        customers: (1..=CLIENT_SIZE)
            .map(|i| {
                let res = ::alloc::fmt::format(format_args!("user-{0}-or-tenat", i));
                res
            })
            .collect(),
    };
    let encodedd: Vec<u8> = bitcode::encode(&cs);
    File::create("cs.bin").unwrap().write_all(&encodedd).unwrap();
    Ok(())
}
