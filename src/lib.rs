#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::http0_2_0::incoming_handler::{Guest, IncomingRequest, ResponseOutparam};
use bindings::wasi::http0_2_0::types::{Fields, OutgoingBody, OutgoingResponse};

struct Component;

impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        println!("Handling request to {:?}://{:?}/{:?}", request.scheme(), request.authority(), request.path_with_query());

        let calc = bindings::vscode::example::types::Engine::new();
        calc.push_operand(123);
        calc.push_operand(456);
        let total = calc.execute();

        let response = OutgoingResponse::new(Fields::new());
        let body = response.body().unwrap();
        ResponseOutparam::set(response_out, Ok(response));

        let resp_stm = body.write().unwrap();
        resp_stm.write(total.to_string().as_bytes()).unwrap();
        resp_stm.flush().unwrap();
        OutgoingBody::finish(body, None).unwrap();
    }
}

bindings::export!(Component with_types_in bindings);
