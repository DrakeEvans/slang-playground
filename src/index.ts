import { Language } from "@nomicfoundation/slang/language";
import { RuleKind, TokenKind } from "@nomicfoundation/slang/syntax/nodes";
import { ProductionKind } from "@nomicfoundation/slang/syntax/parser";

const language = new Language("0.8.0");
const parseOutput = language.parse(ProductionKind.ContractDefinition, "contract Foo {}");
console.log("file: index.mts:7 ~ parseOutput:", parseOutput);
