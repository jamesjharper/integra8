


use ansi_term::Style;
use ansi_term::Colour::{Black, Red, Green, Yellow};
use integra8_formatters::models::ComponentType;

pub struct TreeBranchStyle {
    pub child: String, 
    pub last_child: String, 
    pub no_child: String, 
    pub no_branch: String, 
}

impl TreeBranchStyle {
    pub fn basic() -> Self {
        Self {
            child: "├── ".to_string(), 
            last_child: "└── ".to_string(), 
            no_child: "│   ".to_string(), 
            no_branch: "    ".to_string(), 
        }
    }

    pub fn basic_gray() -> Self {
        Self {
            child: Black.paint("├── ").to_string(),
            last_child: Black.paint("└── ").to_string(), 
            no_child: Black.paint("│   ").to_string(), 
            no_branch: Black.paint("    ").to_string(), 
        }
    }
}


pub struct ComponentNodeStyle {
    pub pass: String,
    pub failed: String, 
    pub overtime: String, 
    pub skipped: String, 
    pub warning: String
}

impl ComponentNodeStyle {
    pub fn colour(pass: &str, failed: &str, overtime: &str, skipped: &str, warning: &str) -> Self {
        Self {
            pass: Green.paint(pass).to_string(),
            failed: Red.paint(failed).to_string(),
            overtime: Red.paint(overtime).to_string(),
            skipped: Black.paint(skipped).to_string(),
            warning: Yellow.paint(warning).to_string(),
        }
    }
}



pub struct NodeStyle {
    suite: ComponentNodeStyle,
    test: ComponentNodeStyle,
    setup: ComponentNodeStyle,
    tear_down: ComponentNodeStyle
}



impl NodeStyle {
    pub fn utf8_colour() -> Self {
        Self {
            suite: ComponentNodeStyle::colour("○","●","●","○","◑"),
            test: ComponentNodeStyle::colour("□","■","■","□","▧"),
            setup: ComponentNodeStyle::colour("△","▲","▲","△","△"),
            tear_down: ComponentNodeStyle::colour("▽","▼","▼","▽","▽"),
        }
    }


    pub fn icon_style<'a>(&'a self, component_type : &ComponentType) -> &'a ComponentNodeStyle {
        match component_type {
            ComponentType::Suite => &self.suite,
            ComponentType::Test => &self.test,
            ComponentType::Setup => &self.setup,
            ComponentType::TearDown => &self.tear_down,
        }
    }
}




impl TreeBranchStyle {
    pub fn standard() -> Self {
        Self {
            child: "├── ".to_string(), 
            last_child: "└── ".to_string(), 
            no_child: "│   ".to_string(), 
            no_branch: "    ".to_string(), 
        }
    }

    pub fn standard_colour() -> Self {
        Self {
            child: Black.paint("├── ").to_string(),
            last_child: Black.paint("└── ").to_string(), 
            no_child: Black.paint("│   ").to_string(), 
            no_branch: Black.paint("    ").to_string(), 
        }
    }
}



pub struct TreeStyle {
    pub branch: TreeBranchStyle, 
    pub node: NodeStyle,
}

impl TreeStyle {
  pub fn standard_colour() -> Self {
      Self {
        branch: TreeBranchStyle::standard_colour(),
        node: NodeStyle::utf8_colour(),
      }
  }
}