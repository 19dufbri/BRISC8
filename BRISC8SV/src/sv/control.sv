module control(
    input logic [7:0] instruction,
    input logic clock,
    output logic swap_en
);
    logic [3:0] opcode;
    logic [1:0] a_select;
    logic [1:0] b_select;
    logic [3:0] immediate;
    reg [1:0] phase;

    assign opcode = instruction[7:4];
    assign a_select = instruction[3:2];
    assign b_select = instruction[1:0];
    assign immediate = { instruction[5:4], instruction[1:0] };

    always @(clock) begin
        if (clock) begin
            phase <= phase + 1;
        end
    end

    always_comb begin
        case ({phase, clock})
            3'b000: begin
                swap_en = 1;
            end
            default begin
                swap_en = 0;
            end
        endcase
    end

endmodule
