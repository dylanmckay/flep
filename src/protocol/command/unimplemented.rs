macro_rules! define_unimplemented_command
{
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name;

        impl $crate::Command for $name
        {
            fn write_payload(&self, _: &mut ::std::io::Write)
                -> Result<(), $crate::Error> {
                unimplemented!();
            }

            fn read_payload(_: &mut ::std::io::BufRead)
                -> Result<Self, $crate::Error> {
                panic!("received unimplemented command: {}", stringify!($name));
            }

            fn command_name(&self) -> &'static str { stringify!($name) }
        }
    }
}

define_unimplemented_command!(ACCT);
define_unimplemented_command!(ADAT);
define_unimplemented_command!(ALLO);
define_unimplemented_command!(APPE);
define_unimplemented_command!(AUTH);
define_unimplemented_command!(CCC);
define_unimplemented_command!(CONF);
define_unimplemented_command!(CWD);
define_unimplemented_command!(DELE);
define_unimplemented_command!(ENC);
define_unimplemented_command!(EPRT);
define_unimplemented_command!(EPSV);
define_unimplemented_command!(HELP);
define_unimplemented_command!(HOST);
define_unimplemented_command!(LANG);
define_unimplemented_command!(LIST);
define_unimplemented_command!(LPRT);
define_unimplemented_command!(LPSV);
define_unimplemented_command!(MDTM);
define_unimplemented_command!(MIC);
define_unimplemented_command!(MKD);
define_unimplemented_command!(MLSD);
define_unimplemented_command!(MLST);
define_unimplemented_command!(NLST);
define_unimplemented_command!(OPTS);
define_unimplemented_command!(PASS);
define_unimplemented_command!(PBSZ);
define_unimplemented_command!(PROT);
define_unimplemented_command!(REST);
define_unimplemented_command!(RETR);
define_unimplemented_command!(RMD);
define_unimplemented_command!(RNFR);
define_unimplemented_command!(RNTO);
define_unimplemented_command!(SITE);
define_unimplemented_command!(SIZE);
define_unimplemented_command!(SMNT);
define_unimplemented_command!(STAT);
define_unimplemented_command!(STOR);
define_unimplemented_command!(STRU);
define_unimplemented_command!(TYPE);
define_unimplemented_command!(XCUP);
define_unimplemented_command!(XMKD);
define_unimplemented_command!(XPWD);
define_unimplemented_command!(XRCP);
define_unimplemented_command!(XRMD);
define_unimplemented_command!(XRSQ);
define_unimplemented_command!(XSEM);
define_unimplemented_command!(XSEN);
