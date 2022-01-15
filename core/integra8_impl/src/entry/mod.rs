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
    let critical_threshold_seconds_expr = global_attr.take_critical_threshold_seconds();
    let warn_threshold_seconds_expr = global_attr.take_warn_threshold_seconds();

    let use_child_process_expr = global_attr.take_use_child_process();

    let settings_extensions = global_attr.take_settings_extensions();
    let settings_extensions_def = settings_extensions.definition;
    let settings_extensions_type = settings_extensions.structopt_type;

    let console_output_formatter = global_attr.take_console_output_formatter();
    let formatter_factory_type = console_output_formatter.formatter_factory_type;
    let formatter_settings_type = console_output_formatter.formatter_settings_type;

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

        type ExecutionContext<'a> = #integra8_path ::components::ExecutionContext<'a, Parameters>;

        pub mod command_line {
            use super::*;

            #settings_extensions_def


            // Base Parameters

            #[derive(Clone, Debug)]
            pub struct BaseParameters<
                TParametersExtend : #structopt_path ::StructOptInternal,
                TParametersFormatter : #structopt_path ::StructOptInternal
            > {
                pub test_parameters: TestParameters,
                pub app_parameters : TParametersExtend,
                pub formatter_parameters : TParametersFormatter
            }

            impl <
                TParametersExtend,
                TParametersFormatter
            >  #structopt_path ::StructOpt for BaseParameters<TParametersExtend, TParametersFormatter>
            where
                TParametersExtend : #structopt_path ::StructOptInternal,
                TParametersFormatter : #structopt_path ::StructOptInternal
            {
                fn clap<'a, 'b>() -> #structopt_path ::clap::App<'a, 'b> {
                    let app = #structopt_path ::clap::App::new(env!("CARGO_PKG_NAME"));
                    <Self as #structopt_path ::StructOptInternal>::augment_clap(app)
                }

                fn from_clap(matches: &#structopt_path ::clap::ArgMatches) -> Self {
                    BaseParameters {
                        test_parameters: #structopt_path ::StructOpt::from_clap(matches),
                        app_parameters: #structopt_path ::StructOpt::from_clap(matches),
                        formatter_parameters: #structopt_path ::StructOpt::from_clap(matches),
                    }
                }
            }

            impl <
                TParametersExtend,
                TParametersFormatter
            >  #structopt_path ::StructOptInternal for BaseParameters<TParametersExtend, TParametersFormatter>
            where
                TParametersExtend : #structopt_path ::StructOptInternal,
                TParametersFormatter : #structopt_path ::StructOptInternal
            {
                fn augment_clap<'a, 'b>(
                    app: #structopt_path ::clap::App<'a, 'b>,
                ) -> #structopt_path ::clap::App<'a, 'b> {
                    {
                        let app = <TestParameters as #structopt_path ::StructOptInternal>::augment_clap(app);
                        let app = <TParametersExtend as #structopt_path ::StructOptInternal>::augment_clap(app);
                        let app = <TParametersFormatter as #structopt_path ::StructOptInternal>::augment_clap(app);

                        app.version(env!("CARGO_PKG_VERSION"))
                    }
                }
                fn is_subcommand() -> bool {
                    false
                }
            }


            impl <
                TParametersExtend,
                TParametersFormatter
            >  BaseParameters<TParametersExtend, TParametersFormatter>
            where
                TParametersExtend : #structopt_path ::StructOptInternal,
                TParametersFormatter : #structopt_path ::StructOptInternal
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
                pub critical_threshold_seconds: u64,
                pub warn_threshold_seconds: u64,
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
                    ).arg(Arg::with_name("framework:use-child-process")
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
                    ).arg(Arg::with_name("framework:max-concurrency")
                        .takes_value(true)
                        .multiple(false)
                        .required(false)
                        .validator(|s| {
                            if s == "auto" {
                                Ok(())
                            } else {
                                ::std::str::FromStr::from_str(s.as_str())
                                    .map(|_: usize| ())
                                    .map_err(|e| e.to_string())
                            }
                        })
                        .long("framework:max-concurrency")
                        .default_value(#max_concurrency_expr),
                    ).arg(Arg::with_name("default:critical-threshold-seconds")
                        .takes_value(true)
                        .multiple(false)
                        .required(false)
                        .validator(|s| {
                            ::std::str::FromStr::from_str(s.as_str())
                                .map(|_: u64| ())
                                .map_err(|e| e.to_string())
                        })
                        .long("default:critical-threshold-seconds")
                        .default_value(#critical_threshold_seconds_expr),
                    ).arg(Arg::with_name("default:warn-threshold-seconds")
                        .takes_value(true)
                        .multiple(false)
                        .required(false)
                        .validator(|s| {
                            ::std::str::FromStr::from_str(s.as_str())
                                .map(|_: u64| ())
                                .map_err(|e| e.to_string())
                        })
                        .long("default:warn-threshold-seconds")
                        .default_value(#warn_threshold_seconds_expr),
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
                                if s == "auto" {
                                    0 
                                } else {
                                    ::std::str::FromStr::from_str(s).unwrap()
                                }
                            })
                            .unwrap(),
                        critical_threshold_seconds: matches
                            .value_of("default:critical-threshold-seconds")
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                            .unwrap(),
                        warn_threshold_seconds: matches
                            .value_of("default:warn-threshold-seconds")
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                            .unwrap(),
                    }
                }
            }


            impl #integra8_path ::components::TestParameters
            for BaseParameters<
                #settings_extensions_type,
                #formatter_settings_type
            > {

                fn child_process_target<'a>(&'a self) -> Option<&'a str> {
                    self.test_parameters.child_process_target.as_ref().map(String::as_ref)
                }

                fn use_child_processes(&self) -> bool {
                    self.test_parameters.use_child_processes
                }

                fn max_concurrency(&self) -> usize {
                    self.test_parameters.max_concurrency
                }

                fn critical_threshold_seconds(&self) -> u64 {
                    self.test_parameters.critical_threshold_seconds
                }

                fn warn_threshold_seconds(&self) -> u64 {
                    self.test_parameters.warn_threshold_seconds
                }

                fn root_namespace(&self) -> &'static str {
                    super::__ROOT_NAMESPACE
                }
            }

            impl #integra8_path ::formatters::FormatterParameters
            for BaseParameters<
                #settings_extensions_type,
                #formatter_settings_type
            > {
                fn create_formatter(&self) -> Option<Box<dyn #integra8_path ::formatters::OutputFormatter>> {
                    let formatters = < #formatter_factory_type as #integra8_path ::formatters::OutputFormatterFactory>::create(
                        &self.formatter_parameters,
                        self
                    );
                    Some(formatters)
                }
            }
        }
    };

    TokenStream::from(tokens)
}
