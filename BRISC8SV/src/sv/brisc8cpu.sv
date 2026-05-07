module brisc8cpu(
    input logic clock,
    input logic reset,
    input logic data_bus_in,
    output logic [7:0] data_bus_out,
    output logic [7:0] address_out
);
    logic [7:0] main_bus;
    logic [7:0] a_bus;
    logic [7:0] b_bus;

    register_file regs (
        .main_bus (main_bus),
        .reset (reset),
        .clock (clock),
        .a_select (2'b00),  // TODO
        .b_select (2'b00),  // TODO
        .write_select (0),  // TODO
        .write_en (0),      // TODO
        .swap_en (0),       // TODO
        .address_select (0),// TODO
        .inc_pc (0),        // TODO
        .alu_a (a_bus),
        .alu_b (b_bus),
        .address (address_out)
    );

    logic [7:0] alu_out;
    alu alu (
        .a_input (a_bus),
        .b_input (b_bus),
        .immediate (0),     // TODO
        .load_immediate (0),// TODO
        .immediate_hilo (0),// TODO
        .operation_type (0),// TODO
        .data_out (alu_out)
    );
endmodule
