#ifndef _GMATLIB_H_
#define _GMATLIB_H_
#ifdef __cplusplus
#include <tuple>
namespace _ngffi_
{
    extern "C" 
    {
#endif // __cplusplus
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
        /// @return an boolean integer value indicating success (0 on failure)
        unsigned inplace_row_scale(void *ptr, unsigned row, double scalar);

        /// @brief scales all elements of the matrix by the given scalar quantity
        /// @param ptr a raw pointer to the matrix to scale
        /// @param scalar the value to scale the matrix by
        /// @return an boolean integer value indicating success (0 on failure) 
        unsigned inplace_scale(void *ptr, double scalar);

        /// @brief adds the row r1 to the row r2 in an element-wise manner
        /// @param ptr a raw pointer to the matrix to mutate
        /// @param r1 the row to add without mutating
        /// @param r2 the row to mutably add to
        /// @return an boolean integer value indicating success (0 on failure)
        unsigned inplace_row_add(void *ptr, unsigned r1, unsigned r2);

        /// @brief adds the row r1 to the row r2 in an element-wise manner AFTER scaling row r1 by a given value
        /// @param ptr a raw pointer to the matrix to mutate
        /// @param r1 the row to add without mutating
        /// @param r2 the row to mutably add to
        /// @param scalar the quantity to scale row r1 by
        /// @return an boolean integer value indicating success (0 on failure)
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

        /// @brief 
        /// @param ptr 
        /// @return 
        unsigned try_inplace_invert(void *ptr);

        unsigned index_mut_double_matrix(void *ptr, unsigned i, unsigned j, double value);

        double index_double_matrix(void *ptr, unsigned i, unsigned j);

        void *clone_double_matrix(void *ptr);

        void free_double_matrix(void *ptr);
#ifdef __cplusplus
    }
}

namespace ngineer
{
    class Matrix
    {
    private:
        unsigned rows;
        unsigned cols;
        void*    ptr;

        Matrix(unsigned rows, unsigned cols, void *ptr):
            rows(rows),
            cols(cols),
            ptr(ptr)
        {
        }

    public:
        Matrix(unsigned rows, unsigned cols): 
            rows(rows), 
            cols(cols), 
            ptr(_ngffi_::new_double_matrix(rows, cols))
        {
        }

        inline const static Matrix Identity(unsigned n) noexcept
        {   
            return Matrix(n, n, _ngffi_::new_double_identity_matrix(n));
        }

        inline Matrix inplace_row_scale(unsigned row, double scalar)
        {
            unsigned result = _ngffi_::inplace_row_scale(ptr, row, scalar);
            if (!result)
            {
                // TODO: throw exception
            }
        }

        inline void inplace_scale(double scalar)
        {
            unsigned result = _ngffi_::inplace_scale(ptr, scalar);
            if (!result)
            {
                // TODO: throw exception
            }
        }

        inline void inplace_row_add(unsigned r1, unsigned r2)
        {
            unsigned result = _ngffi_::inplace_row_add(ptr, r1, r2);
            if (!result)
            {
                // TODO: throw exception
            }
        }

        inline void inplace_scaled_row_add(unsigned r1, unsigned r2, double scalar)
        {
            unsigned result = _ngffi_::inplace_scaled_row_add(ptr, r1, r2, scalar);
            if (!result)
            {
                // TODO: throw exception
            }
        }

        inline Matrix operator*(const Matrix rhs) const
        {
            void* matrixProduct = _ngffi_::multiply_matrix(ptr, const_cast<Matrix *>(&rhs));
            if (matrixProduct == nullptr)
            {
                // TODO: throw exception
            }
            return Matrix(rows, rhs.cols, matrixProduct);
        }

        inline Matrix operator|(const Matrix rhs) const
        {
            void *augmentMatrix = _ngffi_::augment_with(ptr, const_cast<Matrix *>(&rhs));
            if (augmentMatrix == nullptr)
            {
                // TODO: throw exception
            }
            return Matrix(rows, cols + rhs.cols, augmentMatrix);
        }

        inline Matrix subset(unsigned r1, unsigned c1, unsigned r2, unsigned c2) const
        {
            void *matrixSlice = _ngffi_::subset(ptr, r1, c1, r2, c2);
            if (matrixSlice == nullptr)
            {
                // TODO: throw exception
            }
            return Matrix(r2 - r1 + 1, c2 - c1 + 1, matrixSlice);
        }

        inline double trace() const noexcept
        {
            return _ngffi_::trace(ptr);
        }

        inline Matrix transpose() const noexcept
        {
            void *transposed = _ngffi_::transpose(ptr);
            return Matrix(cols, rows, ptr);
        }

        inline void inplace_invert()
        {
            unsigned result = _ngffi_::try_inplace_invert(ptr);
            if (!result)
            {
                // TODO: throw exception
            }
        }

        // TODO make this work with pointers for mutability
        inline double operator[](std::pair<unsigned, unsigned> indices)
        {
            return _ngffi_::index_double_matrix(ptr, indices.first, indices.second);
        }

        ~Matrix()
        {
            _ngffi_::free_double_matrix(ptr);
        }
    };
}
#endif // __cplusplus
#endif // _GMATLIB_H_