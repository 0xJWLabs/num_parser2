use std::collections::HashMap;

pub mod settings;

use crate::objects::Expression;

use self::settings::Rounding;

/// Contains user-defined functions and constants.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Context {
    /// Function declared by the user at runtime.
    pub functions: HashMap<String, (Vec<String>, Box<Expression>)>,
    /// Variables declared by the user at runtime.
    pub variables: HashMap<String, Box<Expression>>,

    // Settings
    /// The decimal digits to display.
    pub rounding: settings::Rounding,
    /// The angle unit to use.
    pub angle_unit: settings::AngleUnit,
    /// Depth limit for recursion control. .
    pub depth_limit: settings::DepthLimit,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
            rounding: settings::Rounding::default(),
            angle_unit: settings::AngleUnit::default(),
            depth_limit: settings::DepthLimit::default(),
        }
    }
}

impl Context {
    /// Generates an empty context.
    pub fn new(
        rounding: Rounding,
        angle_unit: settings::AngleUnit,
        depth_limit: settings::DepthLimit,
    ) -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
            rounding,
            angle_unit,
            depth_limit,
        }
    }

    /// Add all the functions and variables of another context to this one.
    pub fn join_with(&mut self, context: &Self) {
        for (identifier, (params, body)) in context.functions.clone() {
            self.add_function(identifier, params, body);
        }
        for (identifier, expression) in context.variables.clone() {
            self.add_variable(identifier, expression)
        }
    }

    /// Add a function to the user-defined ones.
    pub fn add_function(&mut self, identifier: String, params: Vec<String>, body: Box<Expression>) {
        self.functions.insert(identifier, (params, body));
    }

    /// Add a variable to the user-defined ones.
    pub fn add_variable(&mut self, identifier: String, expression: Box<Expression>) {
        self.variables.insert(identifier, expression);
    }

    /// Returns a user-defined function given an identifier.
    pub fn get_function(&self, identifier: &str) -> Option<(Vec<String>, Box<Expression>)> {
        self.functions.get(identifier).cloned()
    }

    /// Returns a user-defined constant given an identifier.
    pub fn get_var(&self, identifier: &str) -> Option<Box<Expression>> {
        self.variables.get(identifier).cloned()
    }

    /// Returns true if the identifier refers to a user-defined function.
    pub fn is_function(&self, identifier: &str) -> bool {
        self.get_function(identifier).is_some()
    }

    /// Returns true if the identifier refers to a user-defined constant.
    pub fn is_var(&self, identifier: &str) -> bool {
        self.get_var(identifier).is_some()
    }
}
