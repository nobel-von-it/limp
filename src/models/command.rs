pub enum Action {
    Init {
        name: String,

        dependencies: Option<Vec<String>>,
    },

    NewDependency {
        name: String,

        version: Option<String>,

        features: Option<Vec<String>>,

        path_to_snippet: Option<String>,
    },

    Delete {
        name: String,
    },

    Add {
        name: String,
    },

    Link {
        name: String,

        path_to_snippet: String,
    },

    Unlink {
        name: String,
    },

    Update,

    List,
}
