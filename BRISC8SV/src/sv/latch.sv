module latch(
    input d,
    input clock,
    input clear,
    output reg q
);

    always @ (clock or clear)
        if (!clear)
            q <= 0;
        else if (clock)
            q <= d;

endmodule
