1 => Some(Instructions::LDA(Instruction {
        addressing_mode: AddressingModes::Immediate,
    })),

2 => Some(Instructions::LDA(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

3 => Some(Instructions::LDA(Instruction {
        addressing_mode: AddressingModes::IndirectB,
    })),

4 => Some(Instructions::LDA(Instruction {
        addressing_mode: AddressingModes::IndirectC,
    })),

5 => Some(Instructions::LDA(Instruction {
        addressing_mode: AddressingModes::IndirectX,
    })),

6 => Some(Instructions::LDA(Instruction {
        addressing_mode: AddressingModes::IndirectY,
    })),

7 => Some(Instructions::LDA(Instruction {
        addressing_mode: AddressingModes::AbsoluteIndex,
    })),

8 => Some(Instructions::LDA(Instruction {
        addressing_mode: AddressingModes::AbsoluteProperty,
    })),

9 => Some(Instructions::LDA(Instruction {
        addressing_mode: AddressingModes::Parameter,
    })),

10 => Some(Instructions::LDB(Instruction {
        addressing_mode: AddressingModes::Immediate,
    })),

11 => Some(Instructions::LDB(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

12 => Some(Instructions::LDB(Instruction {
        addressing_mode: AddressingModes::IndirectA,
    })),

13 => Some(Instructions::LDB(Instruction {
        addressing_mode: AddressingModes::IndirectC,
    })),

14 => Some(Instructions::LDB(Instruction {
        addressing_mode: AddressingModes::IndirectX,
    })),

15 => Some(Instructions::LDB(Instruction {
        addressing_mode: AddressingModes::IndirectY,
    })),

16 => Some(Instructions::LDB(Instruction {
        addressing_mode: AddressingModes::AbsoluteIndex,
    })),

17 => Some(Instructions::LDB(Instruction {
        addressing_mode: AddressingModes::AbsoluteProperty,
    })),

18 => Some(Instructions::LDB(Instruction {
        addressing_mode: AddressingModes::Parameter,
    })),

19 => Some(Instructions::LDC(Instruction {
        addressing_mode: AddressingModes::Immediate,
    })),

20 => Some(Instructions::LDC(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

21 => Some(Instructions::LDC(Instruction {
        addressing_mode: AddressingModes::IndirectA,
    })),

22 => Some(Instructions::LDC(Instruction {
        addressing_mode: AddressingModes::IndirectB,
    })),

23 => Some(Instructions::LDC(Instruction {
        addressing_mode: AddressingModes::IndirectX,
    })),

24 => Some(Instructions::LDC(Instruction {
        addressing_mode: AddressingModes::IndirectY,
    })),

25 => Some(Instructions::LDC(Instruction {
        addressing_mode: AddressingModes::AbsoluteIndex,
    })),

26 => Some(Instructions::LDC(Instruction {
        addressing_mode: AddressingModes::AbsoluteProperty,
    })),

27 => Some(Instructions::LDC(Instruction {
        addressing_mode: AddressingModes::Parameter,
    })),

28 => Some(Instructions::LDX(Instruction {
        addressing_mode: AddressingModes::Immediate,
    })),

29 => Some(Instructions::LDX(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

30 => Some(Instructions::LDX(Instruction {
        addressing_mode: AddressingModes::IndirectA,
    })),

31 => Some(Instructions::LDX(Instruction {
        addressing_mode: AddressingModes::IndirectB,
    })),

32 => Some(Instructions::LDX(Instruction {
        addressing_mode: AddressingModes::IndirectC,
    })),

33 => Some(Instructions::LDX(Instruction {
        addressing_mode: AddressingModes::IndirectY,
    })),

34 => Some(Instructions::LDX(Instruction {
        addressing_mode: AddressingModes::AbsoluteIndex,
    })),

35 => Some(Instructions::LDX(Instruction {
        addressing_mode: AddressingModes::AbsoluteProperty,
    })),

36 => Some(Instructions::LDX(Instruction {
        addressing_mode: AddressingModes::Parameter,
    })),

37 => Some(Instructions::LDY(Instruction {
        addressing_mode: AddressingModes::Immediate,
    })),

38 => Some(Instructions::LDY(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

39 => Some(Instructions::LDY(Instruction {
        addressing_mode: AddressingModes::IndirectA,
    })),

40 => Some(Instructions::LDY(Instruction {
        addressing_mode: AddressingModes::IndirectB,
    })),

41 => Some(Instructions::LDY(Instruction {
        addressing_mode: AddressingModes::IndirectC,
    })),

42 => Some(Instructions::LDY(Instruction {
        addressing_mode: AddressingModes::IndirectX,
    })),

43 => Some(Instructions::LDY(Instruction {
        addressing_mode: AddressingModes::AbsoluteIndex,
    })),

44 => Some(Instructions::LDY(Instruction {
        addressing_mode: AddressingModes::AbsoluteProperty,
    })),

45 => Some(Instructions::LDY(Instruction {
        addressing_mode: AddressingModes::Parameter,
    })),

46 => Some(Instructions::STA(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

47 => Some(Instructions::STA(Instruction {
        addressing_mode: AddressingModes::Immediate,
    })),

48 => Some(Instructions::STA(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

49 => Some(Instructions::STA(Instruction {
        addressing_mode: AddressingModes::AbsoluteIndex,
    })),

50 => Some(Instructions::STA(Instruction {
        addressing_mode: AddressingModes::AbsoluteProperty,
    })),

51 => Some(Instructions::STA(Instruction {
        addressing_mode: AddressingModes::Parameter,
    })),

52 => Some(Instructions::STB(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

53 => Some(Instructions::STB(Instruction {
        addressing_mode: AddressingModes::Immediate,
    })),

54 => Some(Instructions::STB(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

55 => Some(Instructions::STB(Instruction {
        addressing_mode: AddressingModes::AbsoluteIndex,
    })),

56 => Some(Instructions::STB(Instruction {
        addressing_mode: AddressingModes::AbsoluteProperty,
    })),

57 => Some(Instructions::STB(Instruction {
        addressing_mode: AddressingModes::Parameter,
    })),

58 => Some(Instructions::STC(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

59 => Some(Instructions::STC(Instruction {
        addressing_mode: AddressingModes::Immediate,
    })),

60 => Some(Instructions::STC(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

61 => Some(Instructions::STC(Instruction {
        addressing_mode: AddressingModes::AbsoluteIndex,
    })),

62 => Some(Instructions::STC(Instruction {
        addressing_mode: AddressingModes::AbsoluteProperty,
    })),

63 => Some(Instructions::STC(Instruction {
        addressing_mode: AddressingModes::Parameter,
    })),

64 => Some(Instructions::STX(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

65 => Some(Instructions::STX(Instruction {
        addressing_mode: AddressingModes::Immediate,
    })),

66 => Some(Instructions::STX(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

67 => Some(Instructions::STX(Instruction {
        addressing_mode: AddressingModes::AbsoluteIndex,
    })),

68 => Some(Instructions::STX(Instruction {
        addressing_mode: AddressingModes::AbsoluteProperty,
    })),

69 => Some(Instructions::STY(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

70 => Some(Instructions::STY(Instruction {
        addressing_mode: AddressingModes::Immediate,
    })),

71 => Some(Instructions::STY(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

72 => Some(Instructions::STY(Instruction {
        addressing_mode: AddressingModes::AbsoluteIndex,
    })),

73 => Some(Instructions::STY(Instruction {
        addressing_mode: AddressingModes::AbsoluteProperty,
    })),

74 => Some(Instructions::STY(Instruction {
        addressing_mode: AddressingModes::Parameter,
    })),

75 => Some(Instructions::EQ(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

76 => Some(Instructions::NE(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

77 => Some(Instructions::GT(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

78 => Some(Instructions::LT(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

79 => Some(Instructions::GQ(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

80 => Some(Instructions::LQ(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

81 => Some(Instructions::AND(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

82 => Some(Instructions::OR(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

83 => Some(Instructions::ADD(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

84 => Some(Instructions::SUB(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

85 => Some(Instructions::MUL(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

86 => Some(Instructions::EXP(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

87 => Some(Instructions::DIV(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

88 => Some(Instructions::MOD(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

89 => Some(Instructions::INC(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

90 => Some(Instructions::DEC(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

91 => Some(Instructions::JMP(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

92 => Some(Instructions::CALL(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

93 => Some(Instructions::RET(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

94 => Some(Instructions::RET(Instruction {
        addressing_mode: AddressingModes::Immediate,
    })),

95 => Some(Instructions::RET(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

96 => Some(Instructions::RET(Instruction {
        addressing_mode: AddressingModes::IndirectA,
    })),

97 => Some(Instructions::RET(Instruction {
        addressing_mode: AddressingModes::IndirectB,
    })),

98 => Some(Instructions::RET(Instruction {
        addressing_mode: AddressingModes::IndirectC,
    })),

99 => Some(Instructions::RET(Instruction {
        addressing_mode: AddressingModes::IndirectX,
    })),

100 => Some(Instructions::RET(Instruction {
        addressing_mode: AddressingModes::IndirectY,
    })),

101 => Some(Instructions::RET(Instruction {
        addressing_mode: AddressingModes::AbsoluteIndex,
    })),

102 => Some(Instructions::RET(Instruction {
        addressing_mode: AddressingModes::AbsoluteProperty,
    })),

103 => Some(Instructions::UGR(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

104 => Some(Instructions::ULR(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

105 => Some(Instructions::ULR(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

106 => Some(Instructions::PUSH(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

107 => Some(Instructions::PUSH(Instruction {
        addressing_mode: AddressingModes::IndirectA,
    })),

108 => Some(Instructions::PUSH(Instruction {
        addressing_mode: AddressingModes::IndirectB,
    })),

109 => Some(Instructions::PUSH(Instruction {
        addressing_mode: AddressingModes::IndirectC,
    })),

110 => Some(Instructions::PUSH(Instruction {
        addressing_mode: AddressingModes::IndirectX,
    })),

111 => Some(Instructions::PUSH(Instruction {
        addressing_mode: AddressingModes::IndirectY,
    })),

112 => Some(Instructions::PUSH(Instruction {
        addressing_mode: AddressingModes::AbsoluteIndex,
    })),

113 => Some(Instructions::PUSH(Instruction {
        addressing_mode: AddressingModes::AbsoluteProperty,
    })),

114 => Some(Instructions::LEN(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

115 => Some(Instructions::A2I(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

116 => Some(Instructions::A2F(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

117 => Some(Instructions::A2D(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

118 => Some(Instructions::A2B(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

119 => Some(Instructions::A2S(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

120 => Some(Instructions::A2C(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

121 => Some(Instructions::A2O(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

122 => Some(Instructions::JMPA(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

123 => Some(Instructions::POPS(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

124 => Some(Instructions::ACP(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

125 => Some(Instructions::BRK(Instruction {
        addressing_mode: AddressingModes::Implicit,
    })),

126 => Some(Instructions::CALLN(Instruction {
        addressing_mode: AddressingModes::Immediate,
    })),

127 => Some(Instructions::CO(Instruction {
        addressing_mode: AddressingModes::Absolute,
    })),

128 => Some(Instructions::FN(Instruction {
        addressing_mode: AddressingModes::Immediate,
    })),

129 => Some(Instructions::CALLC(Instruction {
        addressing_mode: AddressingModes::Immediate,
    })),
