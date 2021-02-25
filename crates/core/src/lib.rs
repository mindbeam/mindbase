pub mod error;
pub mod service;
pub mod xport;

pub use error::Error;

#[cfg(test)]
mod tests {
    use toboggan_kv::adapter::BTreeAdapter;

    #[test]
    fn dump() -> Result<(), Error> {
        let mb = Service::new(BTreeAdapter::new())?;

        // Make the ULID play nice for out-of-order testing
        let dur = std::time::Duration::from_millis(50);

        let s1 = mb.alledge(Text::new("Saturday"))?;
        println!("1 {}", s1);
        std::thread::sleep(dur);
        let s2 = mb.alledge(Text::new("Saturday"))?;
        println!("2 {}", s2);
        std::thread::sleep(dur);
        let s3 = mb.alledge(Text::new("Saturday"))?;

        println!("3 {}", s3);
        std::thread::sleep(dur);
        let _f1 = mb.alledge(Text::new("Night's alright for fighting"))?;

        // TODO 2 - change these to use grounding_symbol:
        let dow = mb.alledge(Text::new("Abstract day of the week"))?;
        let alright = mb.alledge(Text::new("Days that are alright for figting in the evening"))?;

        mb.alledge(Analogy::associative(s1.subjective(), dow.subjective()))?;
        mb.alledge(Analogy::associative(s2.subjective(), dow.subjective()))?;
        mb.alledge(Analogy::associative(s3.subjective(), dow.subjective()))?;

        mb.alledge(Analogy::associative(s1.subjective(), alright.subjective()))?;
        mb.alledge(Analogy::associative(s2.subjective(), alright.subjective()))?;
        mb.alledge(Analogy::associative(s3.subjective(), alright.subjective()))?;

        let stdout = std::io::stdout();
        let handle = stdout.lock();

        crate::xport::dump_json(&mb, handle).unwrap();
        Ok(())
    }
    #[test]
    fn load() -> Result<(), std::io::Error> {
        let dump = r#"{"Artifact":["MTEmz+3sCpCnSrrvKJglWvWIEOAJ4Ger7cecqz+/p1I",{"FlatText":{"text":"Days that are alright for figting in the evening"}}]}
        {"Artifact":["QAh0LMMPHQMGJhLNdKH1OasSuCTmS9g2xdViW1gmpJ4",{"FlatText":{"text":"Night's alright for fighting"}}]}
        {"Artifact":["Wtw2TYgjfvmTVazfM0IDhkfwlJFUZ9w0xhNc1H0ilRc",{"FlatText":{"text":"Saturday"}}]}
        {"Artifact":["ZneF/bMFv7Fx4r4eU5NTXJPPBSxrUzWLLZ+jFcm8GAs",{"FlatText":{"text":"English words"}}]}
        {"Artifact":["4Hc9t2ownv7e+hAfzn2f+36xwqKxZWCGJIxKAGQb2KQ",{"FlatText":{"text":"Abstract day of the week"}}]}
        {"Allegation":["AXDCW2fEtID9DIZzkMgQvg",{"id":"AXDCW2fEtID9DIZzkMgQvg","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Artifact":"Wtw2TYgjfvmTVazfM0IDhkfwlJFUZ9w0xhNc1H0ilRc"},"signature":"Alan4mqUTsRW/hEoqaJRRo7CAwx20go75qny4AytRS1a8nrEl6NAvorbw8XKTS9J+3BSVF5ybsICVP/HhRMhDQ"}]}
        {"Allegation":["AXDCW2dnN7VS4wpoUWGJMw",{"id":"AXDCW2dnN7VS4wpoUWGJMw","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Artifact":"Wtw2TYgjfvmTVazfM0IDhkfwlJFUZ9w0xhNc1H0ilRc"},"signature":"7QGeYfot/6vGZdG33fYEMsbG+0qm2WQpFyWwWyJYaHECuDypksgl55ozPTi6ye8XDCdgxg1/NwiHpjQjNwacAA"}]}
        {"Allegation":["AXDCW2gsaVltJU2vcQFKuQ",{"id":"AXDCW2gsaVltJU2vcQFKuQ","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Artifact":"Wtw2TYgjfvmTVazfM0IDhkfwlJFUZ9w0xhNc1H0ilRc"},"signature":"ChS23ik9E+UdYCQUbQzmiMg9RjVu8pF2vI9VmVD4vOotEdpUMIUM62rjJ4Ne6cJY5Js2BICB/E7OkKWeSeowAA"}]}
        {"Allegation":["AXDCW2iDLmb47/OVfXxIQg",{"id":"AXDCW2iDLmb47/OVfXxIQg","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Artifact":"QAh0LMMPHQMGJhLNdKH1OasSuCTmS9g2xdViW1gmpJ4"},"signature":"ltKdtKpe8ZVKFrTOJ4u3C5i6e3Gute2whoSqLTBbz5yedUojIylrxQbXVQDt+rAYSvLOYfZdjKzd2at11qjgDQ"}]}
        {"Allegation":["AXDCW2irj9IxJ/jvXLGmmw",{"id":"AXDCW2irj9IxJ/jvXLGmmw","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Artifact":"4Hc9t2ownv7e+hAfzn2f+36xwqKxZWCGJIxKAGQb2KQ"},"signature":"Rf18ZLD9s4U+kYvqiFOCSwTSvvXOo78/6XxcYM61WYcfYPKSLqYF5nA3gxxqcWNfemqx7S3VRfFSpWv4y5cQAw"}]}
        {"Allegation":["AXDCW2jLdjUVrz/igyanqQ",{"id":"AXDCW2jLdjUVrz/igyanqQ","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Artifact":"MTEmz+3sCpCnSrrvKJglWvWIEOAJ4Ger7cecqz+/p1I"},"signature":"U0Gh4JJReWjQtlttmdeGAwrD2GvkiBxOFfuVZ/rW85Mo7FXXUYplr7mLsGg43/M9xoBmYFt/FjcSf3QCiNRsCA"}]}
        {"Allegation":["AXDCW2jYkL6w/ha0MYDtow",{"id":"AXDCW2jYkL6w/ha0MYDtow","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Analogy":{"left":{"atoms":[{"id":"AXDCW2dnN7VS4wpoUWGJMw","spin":"Up"}],"spread_factor":0.0},"confidence":1.0,"right":{"atoms":[{"id":"AXDCW2irj9IxJ/jvXLGmmw","spin":"Up"}],"spread_factor":0.0}}},"signature":"YHW0oa3AixiAVXUBkUEuFBw52qy/2gfuQoJljyCMrQDc8C3C69uTbarIyfcxJS026qMl/vQCT5JOsjaOZhj/Bg"}]}
        {"Allegation":["AXDCW2j39GJJrNZ9fqXZfQ",{"id":"AXDCW2j39GJJrNZ9fqXZfQ","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Analogy":{"left":{"atoms":[{"id":"AXDCW2fEtID9DIZzkMgQvg","spin":"Up"}],"spread_factor":0.0},"confidence":1.0,"right":{"atoms":[{"id":"AXDCW2irj9IxJ/jvXLGmmw","spin":"Up"}],"spread_factor":0.0}}},"signature":"lnwaN4hP+pEN+Jgnd7EbiPhGIxZE18+iyvAtwNHRyj/7KYxrsMO4EjKl0URn/6AC+7GK0LsS5n6+gaISIpIWBg"}]}
        {"Allegation":["AXDCW2kNM6RcpeAWTOrPWQ",{"id":"AXDCW2kNM6RcpeAWTOrPWQ","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Analogy":{"left":{"atoms":[{"id":"AXDCW2gsaVltJU2vcQFKuQ","spin":"Up"}],"spread_factor":0.0},"confidence":1.0,"right":{"atoms":[{"id":"AXDCW2irj9IxJ/jvXLGmmw","spin":"Up"}],"spread_factor":0.0}}},"signature":"42iAcnBidsgBjqCxNiu4gOtjQkNv/s2ih1Ebeg/27xJQwSeUnLeIyS9ztV4zBx3N97pUzvTVmjXboaZv0+Y3CA"}]}
        {"Allegation":["AXDCW2kaMP6SgxqaYmFegA",{"id":"AXDCW2kaMP6SgxqaYmFegA","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Analogy":{"left":{"atoms":[{"id":"AXDCW2dnN7VS4wpoUWGJMw","spin":"Up"}],"spread_factor":0.0},"confidence":1.0,"right":{"atoms":[{"id":"AXDCW2jLdjUVrz/igyanqQ","spin":"Up"}],"spread_factor":0.0}}},"signature":"7PModUhhvuM8uV5NS8qTcGC+AKvn6KcSdq4hTo52N2ulmwydzml7wzHg33qKttAq2QyErN8iNCl3V5w7wcZ4Aw"}]}
        {"Allegation":["AXDCW2kmqHwYexaNBfcRrw",{"id":"AXDCW2kmqHwYexaNBfcRrw","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Analogy":{"left":{"atoms":[{"id":"AXDCW2fEtID9DIZzkMgQvg","spin":"Up"}],"spread_factor":0.0},"confidence":1.0,"right":{"atoms":[{"id":"AXDCW2jLdjUVrz/igyanqQ","spin":"Up"}],"spread_factor":0.0}}},"signature":"cfUYwSBtIb/qyw7kMdXRadz7/RfxrTKh3lvjXoxbMvlcTUAdsXQPMLapSrpBJ1rw7RD+F/C2+5mmv8PEAINvDA"}]}
        {"Allegation":["AXDCW2k4A7LuPwBx3l2hBw",{"id":"AXDCW2k4A7LuPwBx3l2hBw","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Analogy":{"left":{"atoms":[{"id":"AXDCW2gsaVltJU2vcQFKuQ","spin":"Up"}],"spread_factor":0.0},"confidence":1.0,"right":{"atoms":[{"id":"AXDCW2jLdjUVrz/igyanqQ","spin":"Up"}],"spread_factor":0.0}}},"signature":"dd4fqB3J957G/dP/GUl9lP9ZaTYWqQ5zi5U+3oSniUTOd1rtUX9x6nZENxOa8OnW6571nBRpmyXBOPNnGtDdDg"}]}"#;
        let cursor = std::io::Cursor::new(dump);

        let mb = Service::new(BTreeAdapter::new())?;

        crate::xport::load_json(&mb, cursor).unwrap();

        let _s1 = ClaimId::from_base64("AXDCW2dnN7VS4wpoUWGJMw")?; // this one second
        let _s2 = ClaimId::from_base64("AXDCW2fEtID9DIZzkMgQvg")?; // manipulated the dumpfile above for this one to be recorded first
        let _s3 = ClaimId::from_base64("AXDCW2gsaVltJU2vcQFKuQ")?; // this one third

        let _s = ArtifactId::from_base64("Wtw2TYgjfvmTVazfM0IDhkfwlJFUZ9w0xhNc1H0ilRc")?;

        let genesis_agent_id = AgentId::from_base64("rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY")?;
        mb.add_ground_symbol_agent(&genesis_agent_id)?;

        // let saturdays = mb.get_ground_symbols_for_artifact(&s)?;
        // assert_eq!(saturdays, Some(vec![s1, s2, s3]));

        Ok(())
    }
}
