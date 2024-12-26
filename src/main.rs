use std::io::Write;

use reedline::{DefaultPrompt, Reedline, Signal};
use rust_test::*;

use clap::Parser;

#[derive(Parser)]
#[command(name = "wynd")]
#[command(about = "Wynd is an interactive programming language that can be transpiled to rust, inspired by forth", long_about = None)]
pub struct Cli {
    source: Option<String>,

    /// Tokenize the source code, convert it back into code and print it
    #[clap(long, short)]
    untokenize: bool,

    /// Tokenize the source code and print it to stdout
    #[clap(long, short)]
    tokenize: bool,

    /// Print the stack after each command
    #[clap(long, short)]
    print_stack: bool,
}

pub fn main() {
    let cli = Cli::parse();
    match cli.source.as_ref() {
        Some(source) => {
            execute_arg(&cli, source);
        }
        None => {
            repl(&cli);
        }
    }
}

pub fn execute_arg(cli: &Cli, source: &str) {
    let tokens = tokenize(source);

    if cli.tokenize {
        println!("{:?}", tokens);
        return;
    }

    let mut stack = Vec::new();
    let mut words = Words::new();
    insert_std(&mut words);
    execute_string(cli, source, &mut words, &mut stack);
}

pub fn repl(cli: &Cli) {
    let mut line_editor = Reedline::create();
    let prompt = CustomPrompt::new();

    let mut stack = Vec::new();
    let mut words = Words::new();
    insert_std(&mut words);

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(source)) => {
                if source == "exit" || source == "quit" {
                    break;
                }
                execute_string(cli, &source, &mut words, &mut stack);
                let _ = std::io::stdout().flush();
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
    }
}

pub fn execute_string(cli: &Cli, source: &str, words: &mut Words, stack: &mut Vec<Value>) {
    let tokens = tokenize(source);
    if cli.tokenize {
        println!("{:?}", tokens);
        return;
    }

    let err = execute(&tokens, words, stack);
    if let Err(err) = err {
        println!("Error: {:?}", err);
    } else if cli.untokenize {
        println!("{}", untokenize(&tokens));
    } else if cli.print_stack {
        print_stack(stack);
    }
}
pub struct CustomPrompt {
    default_prompt: DefaultPrompt,
}

impl Default for CustomPrompt {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomPrompt {
    pub fn new() -> Self {
        Self {
            default_prompt: DefaultPrompt::new(
                reedline::DefaultPromptSegment::WorkingDirectory,
                reedline::DefaultPromptSegment::CurrentDateTime,
            ),
        }
    }
}

impl reedline::Prompt for CustomPrompt {
    fn render_prompt_left(&self) -> std::borrow::Cow<str> {
        self.default_prompt.render_prompt_left()
    }

    fn render_prompt_right(&self) -> std::borrow::Cow<str> {
        self.default_prompt.render_prompt_right()
    }

    fn render_prompt_indicator(
        &self,
        _edit_mode: reedline::PromptEditMode,
    ) -> std::borrow::Cow<str> {
        ">".into()
    }

    fn render_prompt_multiline_indicator(&self) -> std::borrow::Cow<str> {
        self.default_prompt.render_prompt_multiline_indicator()
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: reedline::PromptHistorySearch,
    ) -> std::borrow::Cow<str> {
        self.default_prompt
            .render_prompt_history_search_indicator(history_search)
    }
}
