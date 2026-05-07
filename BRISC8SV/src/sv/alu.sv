module alu(
    input logic [7:0] a_input,
    input logic [7:0] b_input,
    input logic [3:0] immediate,
    input logic load_immediate,
    input logic immediate_hilo,
    input logic operation_type,
    output logic [7:0] data_out
);
    always_comb begin
        if (load_immediate) begin
            if (!immediate_hilo) begin
                // low
                assign data_out = { immediate[3], immediate[3], immediate[3], immediate[3], immediate[3:0] };
            end else begin
                // high
                assign data_out = { immediate[3:0], a_input[3:0] };
            end
        end else begin
            if (!operation_type) begin
                // add
                assign data_out = a_input + b_input;
            end else begin
                // nand
                assign data_out = ~(a_input & b_input);
            end
        end
    end
endmodule
