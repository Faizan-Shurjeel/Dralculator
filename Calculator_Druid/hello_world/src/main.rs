use druid::widget::{ Button, Flex, Label };
use druid::{ AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc };

#[derive(Clone, Data, Lens)]
struct CalculatorState {
    input: String,
}

fn build_ui() -> impl Widget<CalculatorState> {
    // Function to create digit buttons
    let digit_button = |digit: char| {
        Button::new(digit.to_string()).on_click(move |_, state: &mut CalculatorState, _| {
            state.input.push(digit);
        })
    };

    // Function to create operator buttons
    let operator_button = |operator: char| {
        Button::new(operator.to_string()).on_click(move |_, state: &mut CalculatorState, _| {
            state.input.push(operator);
        })
    };

    // Layout for the calculator
    Flex::column()
        .with_child(Label::new(|data: &CalculatorState, _: &_| data.input.clone()).padding(10.0))
        .with_child(
            Flex::row()
                .with_child(digit_button('7'))
                .with_child(digit_button('8'))
                .with_child(digit_button('9'))
                .with_child(operator_button('/'))
        )
        .with_child(
            Flex::row()
                .with_child(digit_button('4'))
                .with_child(digit_button('5'))
                .with_child(digit_button('6'))
                .with_child(operator_button('*'))
        )
        .with_child(
            Flex::row()
                .with_child(digit_button('1'))
                .with_child(digit_button('2'))
                .with_child(digit_button('3'))
                .with_child(operator_button('-'))
        )
        .with_child(
            Flex::row()
                .with_child(digit_button('0'))
                .with_child(
                    Button::new("C").on_click(|_, state: &mut CalculatorState, _| {
                        state.input.clear();
                    })
                )
                .with_child(
                    Button::new("=").on_click(|_, state: &mut CalculatorState, _| {
                        let result = calculate(&state.input);
                        state.input = result;
                    })
                )
                .with_child(operator_button('+'))
        )
        .padding(10.0)
}

// Function to evaluate the expression
fn calculate(input: &str) -> String {
    // This is a simple and insecure way to evaluate the expression. It is for demonstration purposes only.
    // Use a proper expression parsing and evaluation library for a real application.
    let result = meval::eval_str(input);
    match result {
        Ok(res) => res.to_string(),
        Err(_) => "Error".to_string(),
    }
}

fn main() {
    let main_window = WindowDesc::new(build_ui).title("Calculator App").window_size((400.0, 400.0));

    AppLauncher::with_window(main_window)
        .launch(CalculatorState {
            input: String::new(),
        })
        .expect("Failed to launch application");
}
