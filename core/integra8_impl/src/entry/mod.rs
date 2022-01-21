use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Path};

mod parameters;
use parameters::ApplicationParameters;

pub fn main_test(input_tokens: TokenStream) -> TokenStream {
    let mut global_attr = parse_macro_input!(input_tokens as ApplicationParameters);

    let integra8_path: Path = parse_quote!(::integra8);
    let structopt_path: Path = parse_quote!(::integra8::structopt);

    let main_expr = global_attr.take_main();

    let max_concurrency_expr = global_attr.take_max_concurrency_expr();

    let setup_critical_threshold_seconds_expr = global_attr.take_setup_critical_threshold_seconds();
    let tear_down_critical_threshold_seconds_expr =
        global_attr.take_tear_down_critical_threshold_seconds();
    let test_critical_threshold_seconds_expr = global_attr.take_test_critical_threshold_seconds();
    let test_warn_threshold_seconds_expr = global_attr.take_test_warn_threshold_seconds();

    let test_concurrency_expr = global_attr.take_test_concurrency();
    let suite_concurrency_expr = global_attr.take_suite_concurrency();

    let use_child_process_expr = global_attr.take_use_child_process();

    let settings_extensions = global_attr.take_settings_extensions();
    let settings_extensions_def = settings_extensions.definition;
    let settings_extensions_type = settings_extensions.structopt_type;

    let console_output_formatter = global_attr.take_console_output_formatter();
    let formatter_factory_type = console_output_formatter.formatter_factory_type;
    let formatter_settings_type = console_output_formatter.formatter_settings_type;

    let console_output_style_expr =
        global_attr.take_console_output_style(&formatter_factory_type, &integra8_path);
    let console_output_level_expr =
        global_attr.take_console_output_level(&formatter_factory_type, &integra8_path);
    let console_output_encoding_expr =
        global_attr.take_console_output_encoding(&formatter_factory_type, &integra8_path);
    let console_output_ansi_mode_expr =
        global_attr.take_console_output_ansi_mode(&formatter_factory_type, &integra8_path);

    let tokens = quote! {

        #main_expr

        use #integra8_path ::decorations::ComponentDecoration;

        #[#integra8_path ::linkme::distributed_slice]
        #[linkme(crate = #integra8_path ::linkme)]
        pub static REGISTERED_COMPONENTS : [fn() -> ComponentDecoration<Parameters>] = [..];

        pub  static __ROOT_NAMESPACE: &'static str = module_path!();

        type Parameters = command_line::BaseParameters<
            command_line:: #settings_extensions_type,
            #formatter_settings_type
        >;

        type ExecutionContext  = #integra8_path ::components::ExecutionContext<Parameters>;

        pub mod command_line {
            use super::*;

            #settings_extensions_def

            // Base Parameters

            #[derive(Clone, Debug)]
            pub struct BaseParameters<
                TParametersExtend : #structopt_path ::StructOptInternal,
                TParametersExtendFormatter : #structopt_path ::StructOptInternal
            > {
                pub test_parameters: TestParameters,
                pub app_parameters : TParametersExtend,
                pub console_output_parameters_ext : TParametersExtendFormatter,
                pub console_output_parameters: ConsoleOutputParameters
            }

            impl <
                TParametersExtend,
                TParametersExtendFormatter
            >  #structopt_path ::StructOpt for BaseParameters<TParametersExtend, TParametersExtendFormatter>
            where
                TParametersExtend : #structopt_path ::StructOptInternal,
                TParametersExtendFormatter : #structopt_path ::StructOptInternal
            {
                fn clap<'a, 'b>() -> #structopt_path ::clap::App<'a, 'b> {
                    let app = #structopt_path ::clap::App::new(env!("CARGO_PKG_NAME"));
                    <Self as #structopt_path ::StructOptInternal>::augment_clap(app)
                }

                fn from_clap(matches: &#structopt_path ::clap::ArgMatches) -> Self {
                    BaseParameters {
                        test_parameters: #structopt_path ::StructOpt::from_clap(matches),
                        app_parameters: #structopt_path ::StructOpt::from_clap(matches),
                        console_output_parameters: #structopt_path ::StructOpt::from_clap(matches),
                        console_output_parameters_ext: #structopt_path ::StructOpt::from_clap(matches),
                    }
                }
            }

            impl <
                TParametersExtend,
                TParametersExtendFormatter
            >  #structopt_path ::StructOptInternal for BaseParameters<TParametersExtend, TParametersExtendFormatter>
            where
                TParametersExtend : #structopt_path ::StructOptInternal,
                TParametersExtendFormatter : #structopt_path ::StructOptInternal
            {
                fn augment_clap<'a, 'b>(
                    app: #structopt_path ::clap::App<'a, 'b>,
                ) -> #structopt_path ::clap::App<'a, 'b> {
                    {
                        let app = <TestParameters as #structopt_path ::StructOptInternal>::augment_clap(app);
                        let app = <TParametersExtend as #structopt_path ::StructOptInternal>::augment_clap(app);
                        let app = <TParametersExtendFormatter as #structopt_path ::StructOptInternal>::augment_clap(app);
                        let app = <ConsoleOutputParameters as #structopt_path ::StructOptInternal>::augment_clap(app);

                        app.version(env!("CARGO_PKG_VERSION"))
                    }
                }
                fn is_subcommand() -> bool {
                    false
                }
            }

            impl <
                TParametersExtend,
                TParametersExtendFormatter
            >  BaseParameters<TParametersExtend, TParametersExtendFormatter>
            where
                TParametersExtend : #structopt_path ::StructOptInternal,
                TParametersExtendFormatter : #structopt_path ::StructOptInternal
            {

                pub fn from_command_line() -> Self {
                    let args: Vec<String> = std::env::args().collect();
                    <Self as #structopt_path ::StructOpt>::from_iter(&args)
                }
            }


            // Test Parameters

            #[derive(Clone, Debug)]
            pub struct TestParameters {
                // indicates is this instance will spawn child processes
                pub use_child_processes: bool,

                pub child_process_target: Option<String>,
                pub max_concurrency: usize,
                pub setup_critical_threshold_seconds: u64,
                pub test_critical_threshold_seconds: u64,
                pub test_warn_threshold_seconds: u64,
                pub tear_down_critical_threshold_seconds: u64,

                pub test_concurrency: #integra8_path ::components::ConcurrencyMode,
                pub suite_concurrency: #integra8_path ::components::ConcurrencyMode
            }

            impl #structopt_path ::StructOptInternal for TestParameters {
                fn augment_clap<'a, 'b>(
                    app: #structopt_path ::clap::App<'a, 'b>,
                ) -> #structopt_path ::clap::App<'a, 'b> {

                    use #structopt_path ::clap::Arg;
                    let app = app
                    .arg(Arg::with_name("internal:child-process-target")
                        .takes_value(true)
                        .hidden(true)
                        .multiple(false)
                        .validator(|s| {
                            ::std::str::FromStr::from_str(s.as_str())
                                .map(|_: String| ())
                                .map_err(|e| e.to_string())
                        })
                        .long("internal:child-process-target"),
                    )
                    .arg(Arg::with_name("framework:use-child-process")
                    .takes_value(true)
                    .multiple(false)
                    .required(false)
                    .validator(|s| {
                        ::std::str::FromStr::from_str(s.as_str())
                            .map(|_: bool| ())
                            .map_err(|e| e.to_string())
                    })
                    .long("framework:use-child-process")
                    .default_value(#use_child_process_expr),
                    )
                    .arg(Arg::with_name("framework:max-concurrency")
                        .takes_value(true)
                        .multiple(false)
                        .required(false)
                        .validator(|s| {
                            if s == "Auto" || s == "Max" {
                                Ok(())
                            } else {
                                ::std::str::FromStr::from_str(s.as_str())
                                    .map(|_: usize| ())
                                    .map_err(|e| e.to_string())
                            }
                        })
                        .long("framework:max-concurrency")
                        .default_value(#max_concurrency_expr),
                    )
                    .arg(Arg::with_name("default:setup-time-limit")
                        .takes_value(true)
                        .multiple(false)
                        .required(false)
                        .validator(|s| {
                            ::std::str::FromStr::from_str(s.as_str())
                                .map(|_: u64| ())
                                .map_err(|e| e.to_string())
                        })
                        .long("default:setup-time-limit")
                        .default_value(#setup_critical_threshold_seconds_expr),
                    )

                    .arg(Arg::with_name("default:tear-down-time-limit")
                        .takes_value(true)
                        .multiple(false)
                        .required(false)
                        .validator(|s| {
                            ::std::str::FromStr::from_str(s.as_str())
                                .map(|_: u64| ())
                                .map_err(|e| e.to_string())
                        })
                        .long("default:tear-down-time-limit")
                        .default_value(#tear_down_critical_threshold_seconds_expr),
                    )
                    .arg(Arg::with_name("default:test-time-limit")
                        .takes_value(true)
                        .multiple(false)
                        .required(false)
                        .validator(|s| {
                            ::std::str::FromStr::from_str(s.as_str())
                                .map(|_: u64| ())
                                .map_err(|e| e.to_string())
                        })
                        .long("default:test-time-limit")
                        .default_value(#test_critical_threshold_seconds_expr),
                    )
                    .arg(Arg::with_name("default:test-warn-time-limit")
                        .takes_value(true)
                        .multiple(false)
                        .required(false)
                        .validator(|s| {
                            ::std::str::FromStr::from_str(s.as_str())
                                .map(|_: u64| ())
                                .map_err(|e| e.to_string())
                        })
                        .long("default:test-warn-time-limit")
                        .default_value(#test_warn_threshold_seconds_expr),
                    )
                    .arg(Arg::with_name("default:test-concurrency")
                        .takes_value(true)
                        .multiple(false)
                        .possible_values(&["Sequential", "Parallel"])
                        .required(false)
                        .validator(|s| {
                            ::std::str::FromStr::from_str(s.as_str())
                                .map(|_: #integra8_path ::components::ConcurrencyMode| ())
                        })
                        .long("default:test-concurrency")
                        .default_value(#test_concurrency_expr),
                    )
                    .arg(Arg::with_name("default:suite-concurrency")
                        .takes_value(true)
                        .multiple(false)
                        .required(false)
                        .possible_values(&["Sequential", "Parallel"])
                        .validator(|s| {
                            ::std::str::FromStr::from_str(s.as_str())
                                .map(|_: #integra8_path ::components::ConcurrencyMode| ())
                        })
                        .long("default:suite-concurrency")
                        .default_value(#suite_concurrency_expr),
                    );
                    app.version(env!("CARGO_PKG_VERSION"))
                }

                fn is_subcommand() -> bool {
                    false
                }
            }

            impl #structopt_path ::StructOpt for TestParameters {
                fn clap<'a, 'b>() -> #structopt_path ::clap::App<'a, 'b> {
                    let app = #structopt_path ::clap::App::new(env!("CARGO_PKG_NAME"));
                    <Self as #structopt_path ::StructOptInternal>::augment_clap(app)
                }

                fn from_clap(matches: &#structopt_path ::clap::ArgMatches) -> Self {
                    TestParameters {
                        child_process_target: matches
                            .value_of("internal:child-process-target")
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap()),
                        use_child_processes: matches
                            .value_of("framework:use-child-process")
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                            .unwrap(),
                        max_concurrency: matches
                            .value_of("framework:max-concurrency")
                            .map(|s| {
                                if s == "Auto" {
                                    #integra8_path ::scheduling::recommended_max_concurrency()
                                } else if s == "Max" {
                                    // Zero indicates that there will be
                                    // no limit will be placed on the 
                                    // number of  components running concurrently 
                                    0 
                                } else {
                                    ::std::str::FromStr::from_str(s).unwrap()
                                }
                            })
                            .unwrap(),
                        setup_critical_threshold_seconds: matches
                            .value_of("default:setup-time-limit")
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                            .unwrap(),
                        tear_down_critical_threshold_seconds: matches
                            .value_of("default:tear-down-time-limit")
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                            .unwrap(),
                        test_critical_threshold_seconds: matches
                            .value_of("default:test-time-limit")
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                            .unwrap(),
                        test_warn_threshold_seconds: matches
                            .value_of("default:test-warn-time-limit")
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                            .unwrap(),
                        test_concurrency: matches
                            .value_of("default:test-concurrency")
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                            .unwrap(),
                        suite_concurrency: matches
                            .value_of("default:suite-concurrency")
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                            .unwrap(),
                    }
                }
            }



            // Test Parameters

            #[derive(Clone, Debug)]
            pub struct ConsoleOutputParameters {
                pub style: String,
                pub detail_level : String,
                pub encoding : String,
                pub ansi_mode : String,
            }

            impl #structopt_path ::StructOptInternal for ConsoleOutputParameters {
                fn augment_clap<'a, 'b>(
                    mut app: #structopt_path ::clap::App<'a, 'b>,
                ) -> #structopt_path ::clap::App<'a, 'b> {

                    let supported_styles = < #formatter_factory_type as #integra8_path ::formatters::OutputFormatterFactory>::supported_styles();
                    let supported_detail_levels = < #formatter_factory_type as #integra8_path ::formatters::OutputFormatterFactory>::supported_detail_levels();
                    let supported_encodings = < #formatter_factory_type as #integra8_path ::formatters::OutputFormatterFactory>::supported_encodings();
                    let supported_ansi = < #formatter_factory_type as #integra8_path ::formatters::OutputFormatterFactory>::supported_ansi_modes();

                    use #structopt_path ::clap::Arg;


                    if !supported_styles.is_empty() {
                        app = app.arg(Arg::with_name("console:style")
                            .takes_value(true)
                            .hidden(false)
                            .multiple(false)
                            .possible_values(&supported_styles)
                            .long("console:style")
                            .default_value(#console_output_style_expr),

                         );
                    }

                    if !supported_detail_levels.is_empty() {
                        app = app.arg(Arg::with_name("console:level")
                            .takes_value(true)
                            .hidden(false)
                            .multiple(false)
                            .possible_values(&supported_detail_levels)
                            .long("console:level")
                            .default_value(#console_output_level_expr),
                        );
                    }

                    if !supported_encodings.is_empty() {
                        app = app.arg(Arg::with_name("console:encoding")
                            .takes_value(true)
                            .hidden(false)
                            .multiple(false)
                            .possible_values(&supported_encodings)
                            .long("console:encoding")
                            .default_value(#console_output_encoding_expr),
                        );
                    }

                    if !supported_ansi.is_empty() {
                        app = app.arg(Arg::with_name("console:ansi-mode")
                            .takes_value(true)
                            .hidden(false)
                            .multiple(false)
                            .possible_values(&supported_ansi)
                            .long("console:ansi-mode")
                            .default_value(#console_output_ansi_mode_expr)
                        );
                    }

                    app.version(env!("CARGO_PKG_VERSION"))
                }

                fn is_subcommand() -> bool {
                    false
                }
            }

            impl #structopt_path ::StructOpt for ConsoleOutputParameters {
                fn clap<'a, 'b>() -> #structopt_path ::clap::App<'a, 'b> {
                    let app = #structopt_path ::clap::App::new(env!("CARGO_PKG_NAME"));
                    <Self as #structopt_path ::StructOptInternal>::augment_clap(app)
                }

                fn from_clap(matches: &#structopt_path ::clap::ArgMatches) -> Self {

                    ConsoleOutputParameters {
                        style: matches
                            .value_of("console:style")
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                            .unwrap_or("".to_string()),
                        detail_level : matches
                            .value_of("console:level")
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                            .unwrap_or("".to_string()),
                        encoding : matches
                            .value_of("console:encoding")
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                            .unwrap_or("".to_string()),
                        ansi_mode : matches
                            .value_of("console:ansi-mode")
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                            .unwrap_or("".to_string()),
                    }
                }
            }

            impl #integra8_path ::components::TestParameters
            for BaseParameters<
                #settings_extensions_type,
                #formatter_settings_type
            > {

                fn child_process_target(&self) -> Option<&'_ str> {
                    self.test_parameters.child_process_target.as_ref().map(String::as_ref)
                }

                fn use_child_processes(&self) -> bool {
                    self.test_parameters.use_child_processes
                }

                fn max_concurrency(&self) -> usize {
                    self.test_parameters.max_concurrency
                }

                fn test_concurrency(&self) -> #integra8_path ::components::ConcurrencyMode {
                    self.test_parameters.test_concurrency.clone()
                }

                fn suite_concurrency(&self) -> #integra8_path ::components::ConcurrencyMode {
                    self.test_parameters.suite_concurrency.clone()
                }

                fn setup_critical_threshold_seconds(&self) -> u64 {
                    self.test_parameters.setup_critical_threshold_seconds
                }
                fn tear_down_critical_threshold_seconds(&self) -> u64 {
                    self.test_parameters.tear_down_critical_threshold_seconds
                }

                fn test_critical_threshold_seconds(&self) -> u64 {
                    self.test_parameters.test_critical_threshold_seconds
                }

                fn test_warn_threshold_seconds(&self) -> u64 {
                    self.test_parameters.test_warn_threshold_seconds
                }

                fn root_namespace(&self) -> &'static str {
                    super::__ROOT_NAMESPACE
                }

                fn console_output_style(&self) -> &'_ str {
                    &self.console_output_parameters.style
                }
                fn console_output_detail_level(&self) -> &'_ str  {
                    &self.console_output_parameters.detail_level
                }
                fn console_output_encoding(&self) -> &'_ str  {
                    &self.console_output_parameters.encoding
                }
                fn console_output_ansi_mode(&self) -> &'_ str {
                    &self.console_output_parameters.ansi_mode
                }
            }

            impl #integra8_path ::formatters::FormatterParameters
            for BaseParameters<
                #settings_extensions_type,
                #formatter_settings_type
            > {
                fn create_formatter(&self) -> Option<Box<dyn #integra8_path ::formatters::OutputFormatter>> {
                    let formatters = < #formatter_factory_type as #integra8_path ::formatters::OutputFormatterFactory>::create(
                        &self.console_output_parameters_ext,
                        self
                    );
                    Some(formatters)
                }
            }
        }
    };

    TokenStream::from(tokens)
}
