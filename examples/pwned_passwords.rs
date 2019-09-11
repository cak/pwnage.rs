fn main() -> Result<(), pwnage::HIBPError> {
    let password = "Password123";
    let user_agent = "GitHub.com/OptionalValue";
    let pwned_password = pwnage::pwned_passwords(password, user_agent)?;
    println!(
        "Pwned?: {:?} | Times: {:?}",
        pwned_password.pwned, pwned_password.times
    );
    Ok(())
}
