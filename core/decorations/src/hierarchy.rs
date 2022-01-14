use indexmap::IndexMap;

use crate::{
    BookEndDecoration, BookEndDecorationPair, ComponentDecoration, SuiteAttributesDecoration,
    TestDecoration,
};

use integra8_components::{
    ComponentDescription, ComponentGeneratorId, Suite, SuiteAttributes, TestParameters,
};

#[derive(Debug)]
pub struct ComponentGroup<TParameters> {
    pub suite: Option<SuiteAttributesDecoration>,
    pub tests: Vec<TestDecoration<TParameters>>,
    pub bookends: Vec<BookEndDecorationPair<TParameters>>,
    pub sub_groups: Vec<ComponentGroup<TParameters>>,
}

impl<TParameters: TestParameters> ComponentGroup<TParameters> {
    pub fn into_components<ComponentsIterator>(
        components: ComponentsIterator,
        parameters: &TParameters,
    ) -> Suite<TParameters>
    where
        ComponentsIterator: IntoIterator<Item = ComponentDecoration<TParameters>>,
    {
        ComponentHierarchy::from_decorated_components(components)
            .into_component_groups()
            .into_component(&mut ComponentGeneratorId::new(), None, parameters)
    }

    fn into_component(
        self,
        id_gen: &mut ComponentGeneratorId,
        parent: Option<(&SuiteAttributes, &ComponentDescription)>,
        parameters: &TParameters,
    ) -> Suite<TParameters> {
        let parent_suite_attributes = self
            .suite
            .unwrap_or_else(|| SuiteAttributesDecoration::root(parameters.root_namespace()));

        let mut suite = parent_suite_attributes.into_component(id_gen, parent, parameters);

        suite.tests = self
            .tests
            .into_iter()
            .map(|x| x.into_component(id_gen, &suite.description, &suite.attributes, parameters))
            .collect();

        suite.bookends = self
            .bookends
            .into_iter()
            .filter(|x| x.has_any())
            .map(|x| x.into_components(id_gen, &suite.description, &suite.attributes))
            .collect();

        suite.suites = self
            .sub_groups
            .into_iter()
            .map(|x| {
                x.into_component(
                    id_gen,
                    Some((&suite.attributes, &suite.description)),
                    parameters,
                )
            })
            .collect();

        suite
    }
}

#[derive(Debug)]
pub struct ComponentHierarchy<TParameters> {
    root: HierarchyNode<TParameters>,
}

impl<TParameters> ComponentHierarchy<TParameters> {
    pub fn new() -> Self {
        Self {
            root: HierarchyNode::new_node(),
        }
    }

    pub fn from_decorated_components<ComponentsIterator>(components: ComponentsIterator) -> Self
    where
        ComponentsIterator: IntoIterator<Item = ComponentDecoration<TParameters>>,
    {
        Self {
            root: components
                .into_iter()
                .fold(HierarchyNode::new_node(), |mut root, c| {
                    root.insert_component(c);
                    root
                }),
        }
    }

    pub fn into_component_groups(self) -> ComponentGroup<TParameters> {
        self.root.into_component_groups()
    }
}

#[derive(Debug)]
pub struct HierarchyNode<TParameters> {
    suite: Option<SuiteAttributesDecoration>,
    tests: Vec<TestDecoration<TParameters>>,
    bookends: BookEndDecorationPair<TParameters>,
    nodes: IndexMap<String, HierarchyNode<TParameters>>,
}

impl<TParameters> HierarchyNode<TParameters> {
    pub fn new_node() -> Self {
        Self {
            suite: None,
            tests: Vec::<TestDecoration<TParameters>>::new(),
            bookends: BookEndDecorationPair::new(),
            nodes: IndexMap::new(),
        }
    }

    pub fn insert_component(&mut self, component: ComponentDecoration<TParameters>) {
        match component {
            ComponentDecoration::IntegrationTest(integration_tst) => {
                self.insert_test(integration_tst);
            }
            ComponentDecoration::Suite(suite_description) => {
                self.insert_suite(suite_description);
            }
            ComponentDecoration::TearDown(bookend) => {
                self.insert_teardown(bookend);
            }
            ComponentDecoration::Setup(bookend) => {
                self.insert_setup(bookend);
            }
        }
    }

    pub fn insert_suite(&mut self, suite: SuiteAttributesDecoration) {
        let mut node = self.find_namespace_entry(&suite.path);
        node.suite = Some(suite);
    }

    pub fn insert_test(&mut self, test: TestDecoration<TParameters>) {
        let node = self.find_method_entry(&test.desc.path);
        node.tests.push(test);
    }

    pub fn insert_setup(&mut self, setup: BookEndDecoration<TParameters>) {
        let mut node = self.find_method_entry(&setup.desc.path);
        // Should raise error when there is already a setup
        node.bookends.setup = Some(setup);
    }

    pub fn insert_teardown(&mut self, teardown: BookEndDecoration<TParameters>) {
        let mut node = self.find_method_entry(&teardown.desc.path);
        // Should raise error when there is already a tear down
        node.bookends.tear_down = Some(teardown);
    }

    fn find_namespace_entry<'a>(&'a mut self, path: &str) -> &'a mut Self {
        let v: Vec<&str> = path.split("::").collect();
        self.find_entry_from_path_elements(&v)
    }

    fn find_method_entry<'a>(&'a mut self, path: &str) -> &'a mut Self {
        let v: Vec<&str> = path.split("::").collect();
        // discard the last element, as the last element is the name of the method
        match v.split_last() {
            None => self,
            Some((_, path)) => self.find_entry_from_path_elements(path),
        }
    }

    fn find_entry_from_path_elements<'a>(&'a mut self, path: &[&str]) -> &'a mut Self {
        match path.split_first() {
            None => self,
            Some((cur, rest)) => {
                let next = self
                    .nodes
                    .entry(cur.to_string())
                    .or_insert(Self::new_node());
                next.find_entry_from_path_elements(rest)
            }
        }
    }

    pub fn into_component_groups(mut self) -> ComponentGroup<TParameters> {
        let mut sub_groups = vec![];
        let suite = std::mem::take(&mut self.suite);
        let mut tests = std::mem::take(&mut self.tests);

        let mut bookends = match self.bookends.has_any() {
            false => vec![],
            true => vec![std::mem::take(&mut self.bookends)],
        };

        for (_, node) in self.nodes {
            let mut group = node.into_component_groups();

            if group.suite.is_some() {
                sub_groups.push(group);
            } else {
                tests.append(&mut group.tests);
                bookends.append(&mut group.bookends);
                sub_groups.append(&mut group.sub_groups);
            }
        }

        return ComponentGroup {
            suite: suite,
            tests: tests,
            bookends: bookends,
            sub_groups: sub_groups,
        };
    }
}
