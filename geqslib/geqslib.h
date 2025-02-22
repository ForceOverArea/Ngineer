#ifndef _GEQSLIB_H_
#define _GEQSLIB_H_
#ifdef __cplusplus
// includes
#include <exception>
#include <sstream>
#include <string>

// uses
using std::runtime_error;
using std::string;
using std::stringstream;

/// @brief contains c-compatible functions for interacting with the Rust geqslib module.
namespace _ngffi_
{
    extern "C"
    {
#endif // __cplusplus

        /// @brief represents whether a `SystemBuilder` object (on the 
        /// Rust-side of the C-FFI code or in C++) is properly constrained,
        /// has yet to be fully constrained, or is over/improperly constrained.
        enum SystemBuilderConstraintStatus
        {
            /// @brief The system is either over-constrained or otherwise 
            /// experienced an error during the constraint-checking process
            ConstraintError = -1,
            /// @brief the system is under-constrained
            NotConstrained  =  0,
            /// @brief the system is properly constrained
            Constrained     =  1,
        };

        /// @brief
        void *new_context_hash_map();

        /// @brief 
        /// @return 
        void *new_default_context_hash_map();

        /// @brief 
        /// @param context 
        /// @param name 
        /// @param val 
        void add_const_to_ctx(void *context, const char *name, double val);

        /// @brief 
        /// @param equation 
        /// @param context 
        /// @param guess 
        /// @param min 
        /// @param max 
        /// @param margin 
        /// @param limit 
        /// @return 
        const char *solve_equation(
            const char *equation, 
            const void *context,
            double guess,
            double min, 
            double max,
            double margin,
            unsigned limit);

        /// @brief 
        /// @param equation 
        /// @param context 
        /// @return 
        const void *new_system_builder(const char *equation, const void *context);

        /// @brief 
        /// @param p_builder 
        /// @param equation 
        /// @return 
        int try_constrain_with(void *p_builder, const char *equation);

        /// @brief 
        /// @param p_builder 
        /// @return 
        int is_fully_constrained(void *p_builder);

        /// @brief 
        /// @param p_builder 
        /// @return 
        const void *build_system(void *p_builder);

        /// @brief 
        /// @param p_builder 
        void debug_system_builder(const void *p_builder);

        /// @brief 
        /// @param p_system 
        /// @param var 
        /// @param guess 
        /// @param min 
        /// @param max 
        /// @return 
        int specify_variable(
            void *p_system, 
            const char *var, 
            double guess, 
            double min, 
            double max);

        /// @brief 
        /// @param p_system 
        /// @param margin 
        /// @param limit 
        /// @return 
        const char *solve_system(void *p_system, double margin, unsigned limit);

        /// @brief 
        /// @param p_context 
        void free_context_hash_map(void *p_context);

        /// @brief 
        /// @param p_builder 
        void free_system_builder(void *p_builder);

        /// @brief 
        /// @param p_system 
        void free_system(void *p_system);

        /// @brief 
        /// @param soln_str 
        void free_solution_string(char *soln_str);

#ifdef __cplusplus
    } // extern "C"
} // namespace _ngffi_

/// @brief contains a C++-compatible class and methods for accessing the Rust geqslib module 
namespace ngineer
{
    using _ngffi_::SystemBuilderConstraintStatus;

    class ContextHashMap
    {
        // Allow SystemBuilder to access raw data
        friend class SystemBuilder;

    private:
        void *ptr;

        ContextHashMap(void *ptr): ptr(ptr)
        {
        }

    public:
        ContextHashMap() noexcept
        {
            ptr = _ngffi_::new_default_context_hash_map();
        }

        inline static ContextHashMap newEmpty() noexcept
        {
            return ContextHashMap { _ngffi_::new_context_hash_map() };
        }

        inline void add_const(string name, double val) noexcept
        {
            _ngffi_::add_const_to_ctx(ptr, name.c_str(), val);
        }

        ~ContextHashMap()
        {
            _ngffi_::free_context_hash_map(ptr);
        }
    }; // class ContextHashMap

    class SystemBuilder
    {
    private:
        void *ptr;

    public:
        SystemBuilder(string equation, ContextHashMap context)
        {
            ptr = const_cast<void *>(
                _ngffi_::new_system_builder(
                    equation.c_str(), context.ptr));
        }

        inline SystemBuilderConstraintStatus try_constrain_with(string equation)
        {
            SystemBuilderConstraintStatus result;
            result = static_cast<SystemBuilderConstraintStatus>(
                _ngffi_::try_constrain_with(ptr, equation.c_str()));

            if (SystemBuilderConstraintStatus::ConstraintError == result)
            {
                // TODO: throw exception
            }

            return result;
        }

        inline SystemBuilderConstraintStatus is_fully_constrained() const noexcept
        {
            return static_cast<SystemBuilderConstraintStatus>(
                _ngffi_::is_fully_constrained(ptr));
        }

        inline ConstrainedSystem build_system()
        {
            const void *result = _ngffi_::build_system(ptr);
            if (result == nullptr)
            {
                // TODO: throw exception
            }
            return ConstrainedSystem { result };
        }

        inline void show_system_builder() const noexcept
        {
            _ngffi_::debug_system_builder(ptr);
        }

        ~SystemBuilder()
        {
            _ngffi_::free_system_builder(ptr);
        }
    }; // class SystemBuilder

    class ConstrainedSystem
    {
        // Allow SystemBuilder access to private constructor 
        friend class SystemBuilder;

    private:
        void *ptr;

        ConstrainedSystem(const void *ptr): ptr(const_cast<void *>(ptr))
        {
        }

    public:
        inline int specify_variable(string var, double guess, double min, double max)
        {
            return _ngffi_::specify_variable(ptr, var.c_str(), guess, min, max);
        }

        inline const char *solve_system(double margin, unsigned limit)
        {
            return _ngffi_::solve_system(ptr, margin, limit);
        }
    }; // class ConstrainedSystem
} // namespace ngineer
#endif // __cplusplus
#endif // _GEQSLIB_H_