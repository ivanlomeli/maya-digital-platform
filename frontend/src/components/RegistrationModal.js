// src/components/RegistrationModal.js - VERSIÓN CORREGIDA

import { useState } from 'react';

export default function RegistrationModal({ onRegister, onLogin }) {
    const [isLogin, setIsLogin] = useState(true);
    const [formData, setFormData] = useState({
        email: '',
        password: '',
        first_name: '',
        last_name: '',
        phone: ''
    });
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [error, setError] = useState('');

    const handleInputChange = (e) => {
        setFormData({
            ...formData,
            [e.target.name]: e.target.value
        });
    };

    const handleSubmit = async (e) => {
        e.preventDefault();
        setIsSubmitting(true);
        setError('');

        // ✅ Validación frontend básica
        if (!formData.email || !formData.password) {
            setError('Email y contraseña son requeridos');
            setIsSubmitting(false);
            return;
        }

        if (!isLogin && (!formData.first_name || !formData.last_name)) {
            setError('Nombre y apellido son requeridos para registro');
            setIsSubmitting(false);
            return;
        }

        if (!isLogin && formData.password.length < 8) {
            setError('La contraseña debe tener al menos 8 caracteres');
            setIsSubmitting(false);
            return;
        }

        try {
            const endpoint = isLogin ? '/api/auth/login' : '/api/auth/register';
            const payload = isLogin 
                ? { 
                    email: formData.email.trim(), 
                    password: formData.password 
                }
                : { 
                    email: formData.email.trim(),
                    password: formData.password,
                    first_name: formData.first_name.trim(),
                    last_name: formData.last_name.trim(),
                    phone: formData.phone?.trim() || null,
                    role: 'Customer' 
                };

            console.log('📤 Enviando solicitud:', { endpoint, payload: { ...payload, password: '[OCULTA]' } });

            // ✅ SOLUCIÓN: URL DIRECTA AL BACKEND (más confiable)
            const response = await fetch(`http://localhost:8080${endpoint}`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(payload),
            });

            console.log('📥 Respuesta recibida:', response.status, response.statusText);

            // ✅ Verificar si la respuesta es JSON válida
            let result;
            const contentType = response.headers.get('content-type');
            if (contentType && contentType.includes('application/json')) {
                result = await response.json();
            } else {
                const textResponse = await response.text();
                console.error('❌ Respuesta no es JSON:', textResponse);
                throw new Error('Respuesta del servidor inválida');
            }

            if (response.ok) {
                console.log('✅ Operación exitosa:', result);
                if (isLogin) {
                    onLogin(result);
                } else {
                    onRegister(result);
                }
                
                // ✅ Limpiar formulario en caso de éxito
                setFormData({
                    email: '',
                    password: '',
                    first_name: '',
                    last_name: '',
                    phone: ''
                });
            } else {
                console.error('❌ Error del servidor:', result);
                setError(result.error || 'Error en la autenticación');
            }
        } catch (error) {
            console.error('❌ Error de conexión:', error);
            setError('Error de conexión. Verifica que el servidor esté funcionando.');
        } finally {
            setIsSubmitting(false);
        }
    };

    return (
        <div className="max-w-md mx-auto">
            <div className="text-center mb-6">
                <h2 className="text-3xl font-bold">
                    {isLogin ? 'Iniciar Sesión' : 'Crear Cuenta'}
                </h2>
                <p className="text-gray-600 mt-2">
                    {isLogin 
                        ? 'Ingresa a tu cuenta de Maya Digital'
                        : 'Únete a Maya Digital'
                    }
                </p>
            </div>

            {error && (
                <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                    <strong>Error:</strong> {error}
                </div>
            )}

            <form onSubmit={handleSubmit} className="space-y-4">
                {!isLogin && (
                    <>
                        <div className="grid grid-cols-2 gap-4">
                            <div>
                                <label className="block text-gray-700 font-semibold mb-2">
                                    Nombre *
                                </label>
                                <input
                                    type="text"
                                    name="first_name"
                                    value={formData.first_name}
                                    onChange={handleInputChange}
                                    className="w-full p-3 border rounded-lg focus:ring-2 focus:ring-blue-500"
                                    required={!isLogin}
                                    disabled={isSubmitting}
                                />
                            </div>
                            <div>
                                <label className="block text-gray-700 font-semibold mb-2">
                                    Apellido *
                                </label>
                                <input
                                    type="text"
                                    name="last_name"
                                    value={formData.last_name}
                                    onChange={handleInputChange}
                                    className="w-full p-3 border rounded-lg focus:ring-2 focus:ring-blue-500"
                                    required={!isLogin}
                                    disabled={isSubmitting}
                                />
                            </div>
                        </div>

                        <div>
                            <label className="block text-gray-700 font-semibold mb-2">
                                Teléfono
                            </label>
                            <input
                                type="tel"
                                name="phone"
                                value={formData.phone}
                                onChange={handleInputChange}
                                className="w-full p-3 border rounded-lg focus:ring-2 focus:ring-blue-500"
                                disabled={isSubmitting}
                                placeholder="Opcional"
                            />
                        </div>
                    </>
                )}

                <div>
                    <label className="block text-gray-700 font-semibold mb-2">
                        Email *
                    </label>
                    <input
                        type="email"
                        name="email"
                        value={formData.email}
                        onChange={handleInputChange}
                        className="w-full p-3 border rounded-lg focus:ring-2 focus:ring-blue-500"
                        required
                        disabled={isSubmitting}
                    />
                </div>

                <div>
                    <label className="block text-gray-700 font-semibold mb-2">
                        Contraseña *
                    </label>
                    <input
                        type="password"
                        name="password"
                        value={formData.password}
                        onChange={handleInputChange}
                        className="w-full p-3 border rounded-lg focus:ring-2 focus:ring-blue-500"
                        required
                        disabled={isSubmitting}
                        minLength={isLogin ? undefined : 8}
                    />
                    {!isLogin && (
                        <p className="text-sm text-gray-500 mt-1">
                            Mínimo 8 caracteres
                        </p>
                    )}
                </div>

                <button
                    type="submit"
                    disabled={isSubmitting}
                    className={`w-full py-3 px-4 rounded-lg font-semibold text-white transition duration-200 ${
                        isSubmitting 
                            ? 'bg-gray-400 cursor-not-allowed' 
                            : 'bg-blue-500 hover:bg-blue-600 focus:ring-2 focus:ring-blue-500'
                    }`}
                >
                    {isSubmitting 
                        ? (isLogin ? 'Iniciando...' : 'Registrando...') 
                        : (isLogin ? 'Iniciar Sesión' : 'Crear Cuenta')
                    }
                </button>
            </form>

            <div className="text-center mt-6">
                <button
                    type="button"
                    onClick={() => {
                        setIsLogin(!isLogin);
                        setError('');
                        setFormData({
                            email: '',
                            password: '',
                            first_name: '',
                            last_name: '',
                            phone: ''
                        });
                    }}
                    className="text-blue-500 hover:text-blue-600 underline"
                    disabled={isSubmitting}
                >
                    {isLogin 
                        ? '¿No tienes cuenta? Crear una' 
                        : '¿Ya tienes cuenta? Iniciar sesión'
                    }
                </button>
            </div>
        </div>
    );
}