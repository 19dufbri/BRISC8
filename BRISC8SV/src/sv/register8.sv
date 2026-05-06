module register8(
    input [7:0] data_in,
    input reset,
    input clock,
    input select,
    output reg [7:0] data_out
);
    always @(reset or clock or select) begin
        if (reset) begin
            data_out <= 0;
        end else if (clock && select) begin
            data_out <= data_in;
        end
    end

endmodule;
