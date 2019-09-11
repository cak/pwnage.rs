# pwnage.rs 

Have I Been Pwned API v3 library

Documentation: [Have I Been Pwned: API v3](https://haveibeenpwned.com/API/v3)

## HIBP API

Options | Description | Arguments
------ | --- | ---
breached_account  |  all breaches for an account (truncated response) | account
breached_account_full  |  all breaches for an account | account
breached_account_by_domain  |  all breaches for an account by domain (truncated response) | account, domain
breached_account_by_domain_full  |  all breaches for an account by domain | account, domain
breaches  |  all breached sites | -
breaches_by_domain  |  breached sites in the system by domain | domain
breach  |  single breached site | name
data_classes  |  all data classes | -
paste_account  |  all pastes for an account | account
pwned_passwords  |  search Pwned Passwords | password

### Example

```Rust
let api_key = String::from("API-KEY"); // HIBP API Key
let user_agent = String::from("GitHub.com/OptionalValue");
let hibp = pwnage::HIBP::new(api_key, user_agent);
let email_address = "name@example.com"; // Email Address
let breaches = hibp.breached_account_full(email_address)?;
	for breach in &breaches {
        println!(
            "Name: {:?} | Title: {:?} | Date: {:?}",
            breach.name, breach.title, breach.breach_date
        );
}
```

## Pwned Passwords

Pwnage sends the first five characters of the password hash to HIBP and receives the suffixes of the matching prefix. Pwnage compares the suffix locally and returns the result. 

Options | Description | Arguments
------ | --- | ---
pwned_passwords  |  search Pwned Passwords | password

### Example

```Rust
let password = "Password123";
let user_agent = "GitHub.com/OptionalValue";
let pwned_password = pwnage::pwned_passwords(password, user_agent)?;
println!(
    "Pwned?: {:?} | Times: {:?}",
    pwned_password.pwned, pwned_password.times
);
```
