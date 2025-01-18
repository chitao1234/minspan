// We want to make sure we are getting the shortest match possible
// without getting tripped up by pathological cases.
pub mod minspan {

    pub fn span<A>(query: &Vec<A>, history: &Vec<A>) -> Option<(usize, usize)>
    where
        A: PartialEq,
    {
        if query.is_empty() {
            return Some((0, 0));
        }
    
        let mut start_indices = vec![None; query.len()]; // Track the start indices for each query element.
        let mut best_span: Option<(usize, usize)> = None;
    
        for (bodyindex, bodychr) in history.iter().enumerate() {
            // Check for each element in the query.
            for keyindex in (0..query.len()).rev() {
                if &query[keyindex] == bodychr {
                    // We found a match for query[keyindex] at bodyindex.
                    start_indices[keyindex] = if keyindex == 0 {
                        // If it's the first character in the query, it starts a potential match.
                        Some(bodyindex)
                    } else {
                        // Otherwise, we extend the match from the previous element.
                        start_indices[keyindex - 1].map(|start| start)
                    };
    
                    // If we have a match for the entire query, update the best span.
                    if keyindex == query.len() - 1 {
                        if let Some(start) = start_indices[query.len() - 1] {
                            let end = bodyindex;
                            let span = (start, end);
    
                            best_span = match best_span {
                                None => Some(span),
                                Some((curr_start, curr_end)) => {
                                    if end - start < curr_end - curr_start {
                                        Some(span)
                                    } else {
                                        Some((curr_start, curr_end))
                                    }
                                }
                            };
                        }
                    }
                }
            }
        }
    
        best_span
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_match() {
        let wrapper = |needle: &str, haystack: &str| match minspan::span(
            &needle.chars().collect(),
            &haystack.chars().collect(),
        ) {
            Some((from, to)) => Some(1 + to - from),
            None => None,
        };

        assert_eq!(wrapper("ab", "ab").unwrap(), 2);
        assert_eq!(wrapper("a", "ab").unwrap(), 1);
        assert_eq!(wrapper("ab", "abc").unwrap(), 2);
        assert_eq!(wrapper("abc", "abcd").unwrap(), 3);
        assert_eq!(wrapper("curl", "curly").unwrap(), 4);
        assert_eq!(wrapper("curl", "acccccurlycurrelly").unwrap(), 4);
        assert_eq!(wrapper("z", "acccccurlycurrelly"), None);
        assert_eq!(wrapper("ssh", "testssh"), Some(3));

        assert_eq!(wrapper("", "abc"), Some(1));

        // Test for empty `haystack`
        assert_eq!(wrapper("abc", ""), None);

        // Test for both `needle` and `haystack` being empty
        assert_eq!(wrapper("", ""), Some(1));

        // Test for a single character match
        assert_eq!(wrapper("a", "a"), Some(1));

        // Test where `needle` is longer than `haystack`
        assert_eq!(wrapper("abc", "a"), None);

        // Test where `needle` is not found in `haystack`
        assert_eq!(wrapper("xyz", "abcdefgh"), None);

        // Test for `needle` appearing multiple times in `haystack`
        assert_eq!(wrapper("ab", "ababc"), Some(2)); // Shortest match
        assert_eq!(wrapper("aba", "abababa"), Some(3)); // Overlapping match

        // Test for `needle` appearing in reversed order in `haystack`
        assert_eq!(wrapper("abc", "cbadefg"), None);

        // Test for non-contiguous matches
        assert_eq!(wrapper("ace", "abcde"), Some(5)); // `ace` appears non-contiguously
        assert_eq!(wrapper("ace", "axxxxxcxxxxxexxxxxx"), Some(13)); // Long gap between matches

        // Test for duplicate characters in `needle`
        assert_eq!(wrapper("aaa", "aaaaaa"), Some(3)); // Shortest span with three consecutive 'a'

        // Test with special characters
        assert_eq!(wrapper("a!b", "a!bc"), Some(3)); // Matches including special characters
        assert_eq!(wrapper("!?*", "abc!?*xyz"), Some(3)); // Special characters match in sequence

        // Test case sensitivity
        assert_eq!(wrapper("abc", "ABC"), None); // Case-sensitive mismatch
        assert_eq!(wrapper("abc", "aBc"), None); // Mixed case-sensitive mismatch

        // Test where `haystack` is very large
        let large_haystack = "a".repeat(1_000_000) + "b";
        assert_eq!(wrapper("ab", &large_haystack), Some(2)); // Match at the end

        // Test with Unicode characters
        assert_eq!(wrapper("こんにちは", "これはこんにちは世界"), Some(5)); // Matches the Japanese substring
        assert_eq!(wrapper("你好", "你好吗"), Some(2)); // Chinese characters match
        assert_eq!(wrapper("😊", "abc😊def"), Some(1)); // Matches emoji
    }
}
