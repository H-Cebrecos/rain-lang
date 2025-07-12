library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

-- Usage: reset before each new crc, input a new data byte each clock cycle, the crc is combinationally available as output in the same cycle.
entity CRC16CCITT is
    Port ( 
        clk : in std_logic;
        rst : in std_logic;
        
        data: in std_logic_vector (7 downto 0);
        valid: in std_logic;
        crc : out std_logic_vector (15 downto 0)
     );
end CRC16CCITT;

architecture Behavioral of CRC16CCITT is
    signal crc_reg : std_logic_vector (15 downto 0);
    signal partial_1 : std_logic_vector (15 downto 0);
    signal partial_2 : std_logic_vector (15 downto 0);
    signal partial_3 : std_logic_vector (15 downto 0);
    signal partial_4 : std_logic_vector (15 downto 0);
begin
    partial_1 <= (crc_reg(7 downto 0) & crc_reg(15 downto 8)) xor "00000000" & data;
    partial_2 <= partial_1 xor ( "000000000000" & partial_1(7 downto 4));
    partial_3 <= partial_2 xor (partial_2(3 downto 0) & "000000000000");
    partial_4 <= partial_3 xor ("000" & partial_3(7 downto 0) & "00000");
    
    crc <= partial_4;
    
    P_reg : process (clk)
    begin
        if rising_edge(clk) then
            if rst = '1' then
                crc_reg <= (others => '0');
            elsif valid = '1' then
                crc_reg <= partial_4;
            end if;
        end if;
    end process;

end Behavioral;

-- CRC Reference C code --
--// Calculates the 16-bit CRC checksum for the given byte sequence.
--unsigned short calculateCRC(unsigned char data[], unsigned int length)
--{
--    unsigned int i;
--    unsigned short crc = 0;
--    for(i=0; i<length; i++){
--        crc = (unsigned char)(crc >> 8) | (crc << 8);
--        crc ^= data[i];
--        crc ^= (unsigned char)(crc & 0xff) >> 4;
--        crc ^= crc << 12;
--        crc ^= (crc & 0x00ff) << 5;
--    }
--    return crc;
--}