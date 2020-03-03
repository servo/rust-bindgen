#ifndef BINDGEN_CLANG_AST_H
#define BINDGEN_CLANG_AST_H

#include "clang/Basic/Version.h"
#include "clang-c/Documentation.h"
#include "clang-c/Index.h"

typedef unsigned long size_t;
typedef unsigned long uint64_t;
typedef long int64_t;

namespace clang {
struct ASTUnit;
struct TargetInfo;
struct CompoundStmt;
struct Decl;
struct Diagnostic;
struct StoredDiagnostic;
struct TranslationUnitDecl;
struct Expr;
struct Type;
struct FileEntry;
struct SourceLocation;
struct CXXBaseSpecifier;
struct ASTContext;
struct SourceRange;
struct Attr;
struct PreprocessedEntity;

namespace comments {
struct Comment;
struct FullComment;
} // namespace comments

#if CLANG_VERSION_MAJOR < 3 || (CLANG_VERSION_MAJOR == 3 && CLANG_VERSION_MINOR <= 8)
// Clang <= 3.8 doesn't include this enum, but we can still expose the same
// functionality
typedef enum {
  CXEval_Int = 1 ,
  CXEval_Float = 2,
  CXEval_ObjCStrLiteral = 3,
  CXEval_StrLiteral = 4,
  CXEval_CFStr = 5,
  CXEval_Other = 6,

  CXEval_UnExposed = 0

} CXEvalResultKind ;
#endif

} // namespace clang

struct EvalResult;

using namespace clang;

struct BindgenStringRef {
  char *s;
  size_t len;
};

#ifndef BINDGEN_IMPLEMENTATION
namespace clang {
struct QualType {
  void *ptr;
};
}
#endif

struct BindgenStringRefSet {
  BindgenStringRef *strings;
  size_t len;

  BindgenStringRefSet() : strings(nullptr), len(0) {}
};


struct BindgenSourceRange {
  SourceLocation *B;
  SourceLocation *E;

  BindgenSourceRange(const SourceRange &range);
  operator bool() const {
    return B && E;
  }
};



void freeString(BindgenStringRef s);
char *cString(BindgenStringRef s);

ASTContext *ASTUnit_getContext(ASTUnit *);
ASTUnit *parseTranslationUnit(const char *source_filename,
                              const char *const *command_line_args,
                              int num_command_line_args, int options,
                              CXUnsavedFile *unsaved_files,
                              unsigned num_unsaved_files);
void disposeASTUnit(ASTUnit *AU);
unsigned ASTUnit_getNumDiagnostics(const ASTUnit *AU);
const StoredDiagnostic *ASTUnit_getDiagnostic(const ASTUnit *AU, unsigned i);
const TargetInfo *ASTUnit_getTargetInfo(ASTUnit *AU);

int TargetInfo_getPointerWidth(const TargetInfo *TI);
BindgenStringRef TargetInfo_getTriple(const TargetInfo *TI);

EvalResult *Expr_Evaluate(const Expr *E, ASTContext *Ctx);
EvalResult *Decl_Evaluate(const Decl *D, ASTContext *Ctx);
CXEvalResultKind EvalResult_getKind(EvalResult *);
double EvalResult_getAsDouble(EvalResult *);
bool EvalResult_isUnsignedInt(EvalResult *);
unsigned long long EvalResult_getAsUnsigned(EvalResult *);
long long EvalResult_getAsLongLong(EvalResult *);
BindgenStringRef EvalResult_getAsStr(EvalResult *);

BindgenStringRef Diagnostic_format(const StoredDiagnostic *);
CXDiagnosticSeverity Diagnostic_getSeverity(const StoredDiagnostic *);

const Decl *getTranslationUnitDecl(ASTUnit *);

bool CursorKind_isInvalid(CXCursorKind kind);

const Decl *Decl_getLexicalParent(const Decl *D);
const Decl *Decl_getSemanticParent(const Decl *D);
const Decl *Decl_getDefinition(const Decl *D, bool isReference);
const Decl *Decl_getReferenced(const Decl *D);
const Decl *Decl_getCanonical(const Decl *D);
const Decl *Decl_getSpecializedTemplate(const Decl *D);
CXCursorKind Decl_getTemplateCursorKind(const Decl *D);
const Decl *Decl_getArgument(const Decl *D, unsigned i);
int Decl_getNumArguments(const Decl *D);
BindgenStringRef Decl_getUSR(const Decl *D);
BindgenStringRef Decl_getSpelling(const Decl *D);
BindgenStringRef Decl_getDisplayName(const Decl *D);
BindgenStringRef Decl_getMangling(const Decl *D, ASTContext *);
BindgenStringRefSet Decl_getCXXManglings(const Decl *D, ASTContext *);
int Decl_getNumTemplateArguments(const Decl *D);
CXCursorKind Decl_getCXCursorKind(const Decl *D);
bool Decl_isDefinition(const Decl *D);
SourceLocation *Decl_getLocation(const Decl *D);
BindgenStringRef Decl_getRawCommentText(const Decl *D, ASTContext *);
comments::Comment *Decl_getParsedComment(const Decl *D, ASTContext *);
QualType Decl_getType(const Decl *D, ASTContext *);
bool Decl_isFunctionInlined(const Decl *D);
int Decl_getFieldDeclBitWidth(const Decl *D, ASTContext *);
QualType Decl_getEnumDeclIntegerType(const Decl *D);
int64_t Decl_getEnumConstantValue(const Decl *D);
uint64_t Decl_getEnumConstantUnsignedValue(const Decl *D);
long long Decl_getOffsetOfField(const Decl *D, ASTContext *);
BindgenSourceRange Decl_getSourceRange(const Decl *D);
QualType Decl_getTypedefDeclUnderlyingType(const Decl *D);
CXLinkageKind Decl_getLinkage(const Decl *D);
CXVisibilityKind Decl_getVisibility(const Decl *D);
CX_CXXAccessSpecifier Decl_getAccess(const Decl *D);
bool CXXField_isMutable(const Decl *D);
bool CXXMethod_isStatic(const Decl *D);
bool CXXMethod_isConst(const Decl *D);
bool CXXMethod_isVirtual(const Decl *D);
bool CXXMethod_isPureVirtual(const Decl *D);
QualType Decl_getResultType(const Decl *D, ASTContext *);


const Expr *Expr_getArgument(const Expr *E, unsigned i);
// const Decl *Expr_getSemanticParent(const Expr *);
int Expr_getNumArguments(const Expr *E);
BindgenStringRef Expr_getUSR(const Expr *E);
BindgenStringRef Expr_getSpelling(const Expr *E);
BindgenStringRef Expr_getDisplayName(const Expr *E);
BindgenStringRef Expr_getMangling(const Expr *E);
BindgenStringRefSet Expr_getCXXManglings(const Expr *E);
CXCursorKind Expr_getCXCursorKind(const Expr *E);
SourceLocation *Expr_getLocation(const Expr *E);
BindgenStringRef Expr_getRawCommentText(const Expr *E);
comments::FullComment *Expr_getParsedComment(const Expr *E);
QualType Expr_getType(const Expr *E);
BindgenSourceRange Expr_getSourceRange(const Expr *E);

const Decl *Type_getDeclaration(QualType);

CXCursorKind Attr_getCXCursorKind(const Attr *);

struct Node {
  CXCursorKind kind;

  union {
    const Decl *decl;
    const Expr *expr;
    const CXXBaseSpecifier *base;
    const Attr *attr;
    const PreprocessedEntity *ppe;
  } ptr;

  Node() : kind(CXCursor_NotImplemented) {}
  Node(const Decl *decl, CXCursorKind kind) : kind(kind) {
    ptr.decl = decl;
  }
  Node(const Expr *expr, CXCursorKind kind) : kind(kind) {
    ptr.expr = expr;
  }
  Node(const CXXBaseSpecifier *base) : kind(CXCursor_CXXBaseSpecifier) {
    ptr.base = base;
  }
  Node(const Attr *attr) : kind(Attr_getCXCursorKind(attr)) {
    ptr.attr = attr;
  }
  Node(const PreprocessedEntity *ppe, CXCursorKind kind) : kind(kind) {
    ptr.ppe = ppe;
  }
  operator bool() const {
    return kind != CXCursor_NotImplemented;
  }
};

typedef CXChildVisitResult (*Visitor)(Node N, Node parent,
                                      ASTUnit *unit,
                                      CXClientData client_data);

void Decl_visitChildren(const Decl *Parent, CXCursorKind kind, Visitor V,
                        ASTUnit *Unit, CXClientData data);
void Expr_visitChildren(const Expr *Parent, CXCursorKind kind, Visitor V,
                        ASTUnit *Unit, CXClientData data);
void CXXBaseSpecifier_visitChildren(const CXXBaseSpecifier *Parent,
                                    CXCursorKind kind, Visitor V, ASTUnit *Unit,
                                    CXClientData data);

void tokenize(ASTUnit *TU, BindgenSourceRange Range, CXToken **Tokens,
              unsigned *NumTokens);
void disposeTokens(const ASTUnit *TU, CXToken *Tokens, unsigned NumTokens);

CXTokenKind getTokenKind(CXToken token);
BindgenStringRef getTokenSpelling(ASTUnit *TU, CXToken token);

CXTypeKind Type_kind(QualType, ASTContext *);
BindgenStringRef Type_getTypeSpelling(QualType, ASTContext *);
bool Type_isConstQualifiedType(QualType);
long long Type_getSizeOf(QualType, ASTContext *);
long long Type_getAlignOf(QualType, ASTContext *);
int Type_getNumTemplateArguments(QualType);
QualType Type_getArgType(QualType T, unsigned index);
int Type_getNumArgTypes(QualType);
QualType Type_getPointeeType(QualType);
QualType Type_getElementType(QualType);
int Type_getNumElements(QualType);
QualType Type_getCanonicalType(QualType, ASTContext *);
bool Type_isFunctionTypeVariadic(QualType);
QualType Type_getResultType(QualType);
CXCallingConv Type_getFunctionTypeCallingConv(QualType);
QualType Type_getNamedType(QualType);
QualType Type_getTemplateArgumentAsType(QualType T, unsigned index);

void getSpellingLocation(ASTUnit *AST, const SourceLocation *T, FileEntry **file, int *line, int *col, int *off);

CXCommentKind Comment_getKind(const comments::Comment *);
unsigned Comment_getNumChildren(const comments::Comment *);
comments::Comment *Comment_getChild(const comments::Comment *, unsigned index);
BindgenStringRef HTMLTagComment_getTagName(const comments::Comment *);
unsigned HTMLStartTag_getNumAttrs(const comments::Comment *);
BindgenStringRef HTMLStartTag_getAttrName(const comments::Comment *, unsigned);
BindgenStringRef HTMLStartTag_getAttrValue(const comments::Comment *, unsigned);

BindgenStringRef CursorKind_getSpelling(CXCursorKind);
BindgenStringRef TypeKind_getSpelling(CXTypeKind);
BindgenStringRef PreprocessedEntity_getSpelling(const PreprocessedEntity *);

BindgenStringRef FileEntry_getName(FileEntry *);

BindgenStringRef getClangVersion();

bool CXXBaseSpecifier_isVirtualBase(const CXXBaseSpecifier *);
QualType CXXBaseSpecifier_getType(const CXXBaseSpecifier *);
BindgenStringRef CXXBaseSpecifier_getSpelling(const CXXBaseSpecifier *);
SourceLocation *CXXBaseSpecifier_getLocation(const CXXBaseSpecifier *);
SourceLocation *Attr_getLocation(const Attr *);
SourceLocation *PreprocessedEntity_getLocation(const PreprocessedEntity *);
BindgenSourceRange CXXBaseSpecifier_getSourceRange(const CXXBaseSpecifier *);
BindgenSourceRange Attr_getSourceRange(const Attr *);
BindgenSourceRange PreprocessedEntity_getSourceRange(const PreprocessedEntity *);

#endif // BINDGEN_CLANG_AST_H