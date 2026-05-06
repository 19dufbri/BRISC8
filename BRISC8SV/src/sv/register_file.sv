module register_file(
    input logic [7:0] main_bus,
    input logic reset,
    input logic clock,
    input logic [1:0] a_select,
    input logic [1:0] b_select,
    input logic [1:0] write_select,
    input logic write_en,
    input logic swap_en,
    input logic address_select,
    input logic inc_pc,
    output logic [7:0] alu_a,
    output logic [7:0] alu_b,
    output logic [7:0] address,

    output logic [31:0] dbg_reg_state
);
    logic [3:0] write_select_decode;
    logic [3:0][7:0] data_in_decode;
    always_comb begin : register_select_decode
        for (int i = 0; i < 4; i++) begin : register_select_decode_loop
            if (inc_pc) begin
                data_in_decode[i] = i[1:0] == 2'b11 ? data_out_decode[3]+1 :  main_bus;
                write_select_decode[i] = i[1:0] == 2'b11 ? 1 : 0;
            end else if (swap_en) begin : register_select_decode_swap
                data_in_decode[i] = (i[1:0] == a_select) ? data_out_decode[b_select] :
                                    (i[1:0] == b_select) ? data_out_decode[a_select] :
                                    main_bus;
                write_select_decode[i] = i[1:0] == a_select || i[1:0] == b_select;
            end else begin : register_select_decode_not_swap
                data_in_decode[i] = main_bus;
                write_select_decode[i] = i[1:0] == write_select ? write_en : 0;
            end
        end
    end

    logic [3:0][7:0] data_out_decode;
    register8 registers[3:0] (
        .data_in (data_in_decode),
        .reset (reset),
        .clock (clock),
        .select (write_select_decode),
        .data_out (data_out_decode)
    );

    assign alu_a = data_out_decode[a_select];
    assign alu_b = data_out_decode[b_select];
    assign address = data_out_decode[address_select ? b_select : 3];

    assign dbg_reg_state = {data_out_decode[3], data_out_decode[2], data_out_decode[1], data_out_decode[0]};
endmodule
