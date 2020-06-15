use crate::features::OptionValueFunction;

pub fn make_feature() -> Vec<(String, OptionValueFunction)> {
    builtin_feature!([
        (
            "color-only",
            bool,
            None,
            _opt => true
        ),
        (
            "commit-decoration-style",
            String,
            None,
            _opt => "none"
        ),
        (
            "commit-style",
            String,
            None,
            _opt => "raw"
        ),
        (
            "file-decoration-style",
            String,
            None,
            _opt => "none"
        ),
        (
            "file-style",
            String,
            None,
            _opt => "raw"
        ),
        (
            "hunk-header-decoration-style",
            String,
            None,
            _opt => "none"
        ),
        (
            "hunk-header-style",
            String,
            None,
            _opt => "raw"
        ),
        (
            "keep-plus-minus-markers",
            bool,
            None,
            _opt => true
        ),
        (
            "tabs",
            usize,
            None,
            _opt => 0
        )
    ])
}
