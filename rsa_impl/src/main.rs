use anyhow::Result;

mod rsa_from_scratch;
mod rsa_using_lib;

fn main() -> Result<()> {
    // rsa_using_lib::run_rsa()?;

    rsa_from_scratch::rsa_from_scratch();

    Ok(())
}
