namespace BRISC8VirtualMachine
{
    public class TimerPeripheral : IPeripheral
    {
        private int _timeScale = 0x1000;
        private int _timer = 0x1000;
        private bool _runTimer;
        public byte? RunCycle()
        {
            if (_runTimer)
                _timer--;
		        
            // If timer is zero, make interrupt
            if (_timer == 0) {
                _timer = _timeScale;
                return 0x00;
            }

            return null;
        }

        public void DoWrite(byte addr, byte value)
        {
            switch (value >> 7)
            {
                case 0b0:
                    // Control register
                    _runTimer = (value & 0x01) == 0x01;
                    break;
                case 0b1:
                    // Timescale register
                    _timeScale = (value & 0x7F) << 5;
                    break;
            }
        }

        public byte DoRead(byte addr)
        {
            return 0x00;
        }
    }
}