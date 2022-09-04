#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/LegacyPassManager.h"

using namespace llvm;

struct Sandbox : public ModulePass {
  static char ID;

  Sandbox() : ModulePass(ID) {}

  bool initialize(Module &M) {
    return true;
  }

  bool runOnModule(Module &M) override {
    for (auto& F: M) {
      runOnFunction(F, M);
    }
    return false;
  }

  bool runOnFunction(Function &F, Module &M) {
    for (auto& BB: F) {
      runOnBasicBlock(BB, M);
    }
    return false;
  }

  bool runOnBasicBlock(BasicBlock &BB, Module &M) {
    for (auto& I: BB) {
      if (auto ci(dyn_cast<CallInst>(&I)); ci) {
        /* Get function name to be called */
        auto func = ci->getCalledFunction();
        if (!func) {
          auto *value = ci->getCalledOperand();
          func = dyn_cast<Function>(value->stripPointerCasts());
        }

        /* Allow these function calls */
        if (func && 
            (func->getName() == "puts"
             || func->getName() == "printf"
             || func->getName() == "__isoc99_scanf"
             || func->getName() == "exit"))
          continue;

        /* Otherwise insert trap */
        std::string str_I;
        raw_string_ostream(str_I) << I;
        IRBuilder<> builder(&I);
        auto msg = builder.CreateGlobalStringPtr(
          "[C-Sandbox] Invalid function call: " + str_I
        );
        auto func_puts = cast<Function>(
          M.getOrInsertFunction("puts",
                                builder.getInt32Ty(),
                                builder.getInt8PtrTy()).getCallee()
        );
        auto func_exit = cast<Function>(
          M.getOrInsertFunction("exit",
                                builder.getVoidTy(),
                                builder.getInt32Ty()).getCallee()
        );
        builder.CreateCall(func_puts, msg);
        builder.CreateCall(func_exit, builder.getInt32(1));
      }
    }

    return false;
  }
};

char Sandbox::ID = 0;
static RegisterPass<Sandbox> X("Sandbox",
                               "Deny dangerous system calls",
                               false,
                               false);
