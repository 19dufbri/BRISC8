module myregister(
    input [7:0] data_in,
    input reset,
    input clock,
    input select,
    output [7:0] data_out
);

    genvar i;

    for (i = 0; i < 8; i++) begin
        latch l0 (
            .d (data_in[i]),
            .clock (clock),
            .clear (reset),
            .q (data_out[i])
        );
    end

endmodule;
