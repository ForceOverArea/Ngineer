#ifndef _GMATLIB_H_
#define _GMATLIB_H_
// includes
#include <limits.h>
#ifdef __cplusplus
#include <exception>
#include <sstream>
#include <tuple>

// uses
using std::pair;
using std::runtime_error;
using std::string;
using std::stringstream;

/// @brief contains c-compatible functions for interacting with the Rust gmatlib module.
namespace _ngffi_
{
    extern "C" 
    {
#endif // __cplusplus
        
        /// @brief represents the possible outcomes of an attempted matrix inversion. 
        ///
        /// ### Rationale:
        ///
        /// This helps to give the user more information about what went wrong during the 
        /// inversion process. Since the `try_inplace_invert` function mutates the matrix, 
        /// there is not good forensic evidence as to what problem occurred. This type serves
        /// to translate the error codes that this function may return in source code for 
        /// readability.
        enum MatrixInversionError
        {
            /// @brief indicates that a determinant of zero was calculated for the given matrix, and it is non-invertible as a result
            DeterminantWasZero,
            /// @brief indicates that the matrix was of size 1 x 1 and contained only zero. This is equivalent to division by zero and is undefined.
            SingularValueWasZero,
            /// @brief indicates that division by zero was required to invert the matrix. This is undefined and prevents the matrix from being invertible as a result.
            ZeroDuringInversion,
            /// @brief indicates that one of any other number of internal gmatlib errors occurred. 
            UnknownGmatlibError,
            
            /// @brief this should be the last value and should have the maximum value a 32-bit integer can have. This value indicates that the process inverted the matrix successfully
            Ok = UINT_MAX,
        };

        /// @brief creates a new matrix of `double` values with the specified number of rows and columns
        /// @param rows the number of rows that the matrix should have
        /// @param cols the number of columns that the matrix should have
        /// @return a raw pointer to the new matrix. 
        void *new_double_matrix(unsigned rows, unsigned cols);

        /// @brief creates a new identity matrix of `double` values with the specified edge length
        /// @param n the edge length of the matrix
        /// @return a raw pointer to the new matrix.
        void *new_double_identity_matrix(unsigned n);

        /// @brief scales the given row of the matrix
        /// @param ptr a raw pointer to the matrix to mutate
        /// @param row the 0-indexed row to scale 
        /// @param scalar the scalar value to multiply the row values by
        /// @return a boolean integer value indicating success (0 on failure)
        unsigned inplace_row_scale(void *ptr, unsigned row, double scalar);

        /// @brief scales all elements of the matrix by the given scalar quantity
        /// @param ptr a raw pointer to the matrix to scale
        /// @param scalar the value to scale the matrix by
        /// @return a boolean integer value indicating success (0 on failure) 
        unsigned inplace_scale(void *ptr, double scalar);

        /// @brief adds the row r1 to the row r2 in an element-wise manner
        /// @param ptr a raw pointer to the matrix to mutate
        /// @param r1 the row to add without mutating
        /// @param r2 the row to mutably add to
        /// @return a boolean integer value indicating success (0 on failure)
        unsigned inplace_row_add(void *ptr, unsigned r1, unsigned r2);

        /// @brief adds the row r1 to the row r2 in an element-wise manner AFTER scaling row r1 by a given value
        /// @param ptr a raw pointer to the matrix to mutate
        /// @param r1 the row to add without mutating
        /// @param r2 the row to mutably add to
        /// @param scalar the quantity to scale row r1 by
        /// @return a boolean integer value indicating success (0 on failure)
        unsigned inplace_scaled_row_add(void *ptr, unsigned r1, unsigned r2, double scalar);

        /// @brief returns the matrix product of the two given matrices
        /// @param ptr_a the left operand
        /// @param ptr_b the right operand
        /// @return the matrix product of the matrices at ptr_a and ptr_b
        void *multiply_matrix(void *ptr_a, void *ptr_b);

        /// @brief returns the augment matrix created by the two given matrices
        /// @param ptr_a the left operand
        /// @param ptr_b the right operand (the matrix whose rows should be appended to the one at ptr_a)
        /// @return a raw pointer to the augment matrix of the two given matrices
        void *augment_with(void *ptr_a, void *ptr_b);

        /// @brief returns a pointer to a new matrix that contains the specified 2D slice of elements from the given matrix
        /// @param ptr a raw pointer to the matrix to slice 
        /// @param r1 the first row to slice
        /// @param c1 the first column to slice
        /// @param r2 the last row in the slice
        /// @param c2 the last column in the slice
        /// @return a pointer to a new matrix containing the specified slice
        void *subset(void *ptr, unsigned r1, unsigned c1, unsigned r2, unsigned c2);

        /// @brief returns the trace of the given matrix
        /// @param ptr a raw pointer to the matrix to compute the trace of
        /// @return the trace of the matrix
        double trace(void *ptr);

        /// @brief creates a new matrix whose elements are the same as the given matrix mirrored about it's diagonal
        /// @param ptr a raw pointer to the matrix to transpose
        /// @return a pointer to a new transposed matrix 
        void *transpose(void *ptr);

        /// @brief attempts to invert the matrix by mutating it in-place
        /// @param ptr a raw pointer to the matrix to invert
        /// @return a boolean integer value indicating success (0 on failure)
        unsigned try_inplace_invert(void *ptr);

        /// @brief mutates the individual `double` value at the `i`th row and `j`th column
        /// @param ptr a raw pointer to the matrix to index 
        /// @param i the row of the element to mutate
        /// @param j the column of the element to mutate
        /// @param value the value that the matrix should have the given index set to
        /// @return a boolean integer value indicating success (0 on failure)
        unsigned index_mut_double_matrix(void *ptr, unsigned i, unsigned j, double value);

        /// @brief returns the `double` element at the `i`th row and the `j`th column
        /// @param ptr a raw pointer to the matrix to index
        /// @param i the row of the element to mutate
        /// @param j the column of the element to mutate
        /// @return the double value at the given index
        double index_double_matrix(void *ptr, unsigned i, unsigned j);

        /// @brief clones the matrix, allocating space sufficient for a copy of the given matrix
        /// @param ptr a raw pointer to the matrix to clone
        /// @return a raw pointer to the new matrix
        void *clone_double_matrix(void *ptr);

        /// @brief frees the memory tied up in the given matrix
        /// @param ptr a raw pointer to the matrix to free
        void free_double_matrix(void *ptr);
#ifdef __cplusplus
    } // extern "C"
} // namespace _ngffi_

/// @brief contains a C++-compatible class and methods for accessing the Rust gmatlib module 
namespace ngineer
{
    /// @brief Represents a matrix quantity of `double` values
    class Matrix
    {
    private:
        /// @brief the number of rows in the given matrix
        unsigned rows;
        /// @brief the number of columns in the given matrix
        unsigned cols;
        /// @brief the raw pointer to the matrix elements
        void *ptr;

        /// @brief a private constructor for building a `Matrix` from raw parts
        /// @param rows the number of rows in the matrix
        /// @param cols the number of columns in the matrix 
        /// @param ptr a raw pointer to the matrix to instantiate
        Matrix(unsigned rows, unsigned cols, void *ptr):
            rows(rows),
            cols(cols),
            ptr(ptr)
        {
        }

    public:
        /// @brief creates a new zero matrix with the specified number of rows and columns
        /// @param rows the number of rows that the matrix should have
        /// @param cols the number of columns that the matrix should have
        Matrix(unsigned rows, unsigned cols): 
            rows(rows), 
            cols(cols), 
            ptr(_ngffi_::new_double_matrix(rows, cols))
        {
        }

        /// @brief creates a new identity matrix with the specified edge length
        /// @param n the number of rows/columns that the matrix should have
        /// @return a new identity matrix
        inline const static Matrix Identity(unsigned n) noexcept
        {   
            return Matrix { n, n, _ngffi_::new_double_identity_matrix(n) };
        }

        /// @brief returns a boolean value indicating whether the given matrix is square
        /// @return whether the matrix is square
        inline bool is_square() const noexcept
        {
            return rows == cols;
        }

        /// @brief scales the given row of the matrix
        /// @param row the 0-indexed row to scale 
        /// @param scalar the scalar value to multiply the row values by
        inline void inplace_row_scale(unsigned row, double scalar)
        {
            unsigned result = _ngffi_::inplace_row_scale(ptr, row, scalar);
            if (!result)
            {
                string errorText;
                stringstream ss;
                ss << "failed to scale row " << row << " by " << scalar << ".\n";
                ss >> errorText;
                throw runtime_error { errorText };
            }
        }

        /// @brief scales the matrix by the given scalar value
        /// @param scalar the quantity to scale the matrix elements by
        inline void inplace_scale(double scalar)
        {
            unsigned result = _ngffi_::inplace_scale(ptr, scalar);
            if (!result)
            {
                string errorText;
                stringstream ss;
                ss << "failed to scale matrix by " << scalar << ".\n";
                ss >> errorText;
                throw runtime_error { errorText };
            }
        }

        /// @brief adds the row `r1` to row `r2` in an element-wise fashion
        /// @param r1 the row to add to another row
        /// @param r2 the row to mutate by the addition operation
        inline void inplace_row_add(unsigned r1, unsigned r2)
        {
            unsigned result = _ngffi_::inplace_row_add(ptr, r1, r2);
            if (!result)
            {
                string errorText;
                stringstream ss;
                ss << "failed to add row " << r1 << " to row " << r2 << ".\n";
                ss >> errorText;
                throw runtime_error { errorText };
            }
        }

        /// @brief scales the row `r1` by `scalar` before adding it to row `r2` in an element-wise fashion
        /// @param r1 the row to scale and add
        /// @param r2 the row to mutate by the scaled addition operation
        /// @param scalar the value to scaled 
        inline void inplace_scaled_row_add(unsigned r1, unsigned r2, double scalar)
        {
            unsigned result = _ngffi_::inplace_scaled_row_add(ptr, r1, r2, scalar);
            if (!result)
            {
                string errorText;
                stringstream ss;
                ss << "failed to add row " << r1 << " scaled by " << scalar << " to row " << r2 << ".\n";
                ss >> errorText;
                throw runtime_error { errorText };
            }
        }

        /// @brief returns the matrix product between two matrices
        /// @param rhs the right-hand operand matrix
        /// @return the matrix product of `this` matrix and `rhs`
        inline Matrix operator*(const Matrix rhs) const
        {
            void* matrixProduct = _ngffi_::multiply_matrix(ptr, const_cast<Matrix *>(&rhs));
            if (matrixProduct == nullptr)
            {
                string errorText;
                stringstream ss;
                ss << "failed to multiply matrix: " << this << " by matrix: " << &rhs << ".\n";
                ss >> errorText;
                throw runtime_error { errorText };
            }
            return Matrix { rows, rhs.cols, matrixProduct };
        }

        /// @brief returns the augment of the two given matrices if their number of rows are equal
        /// @param rhs the right-hand operand matrix
        /// @return a new matrix produced by appending the rows of the right-hand matrix to the rows of the left 
        inline Matrix operator|(const Matrix rhs) const
        {
            void *augmentMatrix = _ngffi_::augment_with(ptr, const_cast<Matrix *>(&rhs));
            if (augmentMatrix == nullptr)
            {
                string errorText;
                stringstream ss;
                ss << "failed to augment matrix: " << this << " with matrix: " << &rhs << ".\n";
                ss >> errorText;
                throw runtime_error { errorText };
            }
            return Matrix { rows, cols + rhs.cols, augmentMatrix };
        }

        /// @brief returns a new matrix containing copies of the values in the given slice of the parent matrix
        /// @param r1 the first row to include in the slice
        /// @param c1 the first column to include in the slice
        /// @param r2 the second row to include in the slice
        /// @param c2 the second column to include in the slice
        /// @return the slice of the parent matrix
        inline Matrix subset(unsigned r1, unsigned c1, unsigned r2, unsigned c2) const
        {
            string errorText;
            stringstream ss;
            void *matrixSlice = _ngffi_::subset(ptr, r1, c1, r2, c2);

            if (r1 >= rows)
            {
                ss << "failed to slice matrix starting at row " << r1 << " because it only has " << rows << " rows.\n";
            }
            else if (r2 >= rows)
            {
                ss << "failed to slice matrix ending at row " << r2 << " because it only has " << rows << " rows.\n";
            }
            else if (c1 >= cols)
            {
                ss << "failed to slice matrix starting at column " << c1 << " because it only has " << cols << " columns.\n";
            }
            else if (c2 >= cols)
            {
                ss << "failed to slice matrix ending at column " << c2 << "because it only has " << cols << " columns.\n";
            }
            else if (matrixSlice == nullptr)
            {
                ss << "failed to slice matrix: "  ".\n";
            }

            ss >> errorText;

            if (!errorText.empty())
            {
                throw runtime_error { errorText };
            }

            return Matrix { r2 - r1 + 1, c2 - c1 + 1, matrixSlice };
        }

        /// @brief returns the trace of the matrix if it is square. If not, then this method returns NaN.
        /// @return either the trace value of the matrix or NaN if the matrix is not square.
        inline double trace() const noexcept
        {
            return _ngffi_::trace(ptr);
        }

        /// @brief produces a copy of the matrix containing the same elements but transposed 
        /// @return the transpose of the given matrix as a new matrix object
        inline Matrix transpose() const noexcept
        {
            void *transposed = _ngffi_::transpose(ptr);
            return Matrix { cols, rows, ptr };
        }

        /// @brief inverts the given matrix by mutating its values through gaussian elimination
        inline void inplace_invert()
        {
            constexpr char *ERROR_MESSAGE_PREFIX = "failed to invert matrix because ";
            unsigned result = _ngffi_::try_inplace_invert(ptr);
            string errorText;
            stringstream ss;

            switch (static_cast<_ngffi_::MatrixInversionError>(result))
            {
            case _ngffi_::MatrixInversionError::DeterminantWasZero:
                errorText = "the given matrix had a determinant of zero.\n";
                break;
            case _ngffi_::MatrixInversionError::SingularValueWasZero:
                errorText = "the given matrix was of size 1 x 1 and contained only zero.\n";
                break;
            case _ngffi_::MatrixInversionError::ZeroDuringInversion:
                errorText = "division by zero occurred during the inversion process.\n";
                break;
            case _ngffi_::MatrixInversionError::UnknownGmatlibError:
                errorText = "an internal gmatlib-defined error occurred.\n";
                break;
            case _ngffi_::MatrixInversionError::Ok:
                return; // ABORT CONTROL FLOW! INVERSION WAS SUCCESSFUL!
                break;
            default: 
                errorText = "an unknown error occurred. please report an issue at: https://github.com/ForceOverArea/Ngineer/issues.\n";
                break;
            }
            ss << ERROR_MESSAGE_PREFIX << errorText;
            errorText.clear(); // TODO: is this necessary?
            ss >> errorText;
            throw runtime_error { errorText };
        }

        /// @brief mutates the value at the given index in the matrix to a desired value.
        /// @param indices the row and column of the element to modify
        /// @param value the value that the chosen element should have
        /// @return a boolean value indicating success. (`false` on failure)
        inline bool set(pair<unsigned, unsigned> indices, double value) noexcept
        {
            return _ngffi_::index_mut_double_matrix(ptr, indices.first, indices.second, value);
        }

        /// @brief returns a copy of the value at the given index in the matrix
        /// @param indices the row and column of the element to get
        /// @return the `double` value located at that index
        inline const double operator[](pair<unsigned, unsigned> indices)
        {
            return _ngffi_::index_double_matrix(ptr, indices.first, indices.second);
        }

        /// @brief formats this matrix reference as part of a `std::stringstream`, allowing users to view it's elements all at once. 
        ///        Elements are all printed as a MATLAB-style matrix, closed by square braces with comma-delimited columns and 
        ///        semicolon-delimited rows.
        /// @param out the stringstream to format this matrix in
        /// @param o the matrix to append to the stringstream with the stream operator `<<`
        /// @return a reference to the given stringstream
        inline friend stringstream& operator<<(stringstream &out, Matrix &const o)
        {
            unsigned numElems = o.rows * o.cols;

            out << '[';

            for (unsigned i = 0; i < o.rows; i++)
            {
                for (unsigned j = 0; j < o.cols; j++)
                {
                    out << o[{ i, j }];
                    if (o.cols - 1 != j) // for all elements but the last one...
                    {
                        out << ','; // ...append a comma
                    }
                }

                if (o.rows - 1 != i) // for all rows but the last one...
                {
                    out << ';'; // ...append a semicolon, MATLAB style
                }
                else
                {
                    out << ']'; // append a bracket after the last row has been printed
                }
            }

            return out;
        }

        /// @brief deallocates the memory tied up in this matrix
        ~Matrix()
        {
            _ngffi_::free_double_matrix(ptr);
        }
    }; // class Matrix
} // namespace ngineer
#endif // __cplusplus
#endif // _GMATLIB_H_