#c TransMat
class TransMat:
    #v properties
    mat : Glm.mat4
    #f __init__
    def __init__(self, mat:Optional[Glm.mat4]=None) -> None:
        if mat is None:
            self.mat = Glm.mat4.create()
            pass
        else:
            self.mat = mat
            pass
        pass
    #f mat4
    def mat4(self) -> Glm.mat4:
        return self.mat
    #f mat_after
    def mat_after(self, pre_mat:"TransMat") -> "TransMat":
        return TransMat(mat=pre_mat.mat * self.mat)
    #f __str__
    def __str__(self) -> str:
        return "[" + ("   ".join([" ".join([str(v) for v in col]) for col in self.mat])) + "]" # type: ignore
    #f All done
    pass

