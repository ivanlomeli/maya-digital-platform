// pages/PortalPage.js - VERSIÓN CORREGIDA CON MANEJO SEGURO DE ARRAYS

import React, { useState, useEffect } from 'react';
import { Navigate, Link } from 'react-router-dom';

function PortalPage({ userData, isLoggedIn }) {
    // ✅ TODOS LOS HOOKS AL INICIO - INICIALIZAR COMO ARRAYS VACÍOS
    const [hotels, setHotels] = useState([]); // ✅ Siempre array
    const [businesses, setBusinesses] = useState([]); // ✅ Siempre array
    const [bookings, setBookings] = useState([]); // ✅ Siempre array
    const [loading, setLoading] = useState(true);
    const [activeTab, setActiveTab] = useState('overview');
    const [error, setError] = useState(null);

    // ✅ FUNCIÓN PARA CARGAR HOTELES CON MANEJO SEGURO
    const fetchHotels = async () => {
        if (!userData || (userData.role !== 'HotelOwner' && userData.role !== 'Admin')) return;
        
        try {
            const token = localStorage.getItem('auth_token');
            const response = await fetch('http://localhost:8080/api/hotels/my-hotels', {
                headers: {
                    'Authorization': `Bearer ${token}`,
                    'Content-Type': 'application/json'
                }
            });

            if (response.ok) {
                const data = await response.json();
                console.log('📥 Respuesta de hoteles:', data);
                
                // ✅ MANEJO SEGURO: Asegurarse de que sea array
                if (data && data.hotels && Array.isArray(data.hotels)) {
                    setHotels(data.hotels);
                } else if (Array.isArray(data)) {
                    setHotels(data);
                } else {
                    console.warn('Respuesta de hoteles no es array:', data);
                    setHotels([]); // Default a array vacío
                }
                console.log('✅ Hoteles cargados correctamente');
            } else {
                console.error('Error cargando hoteles:', response.status);
                setError('Error al cargar hoteles');
                setHotels([]); // ✅ Asegurar array vacío en error
            }
        } catch (error) {
            console.error('Error cargando hoteles:', error);
            setError('Error al cargar hoteles');
            setHotels([]); // ✅ Asegurar array vacío en error
        }
    };

    // ✅ FUNCIÓN PARA CARGAR NEGOCIOS CON MANEJO SEGURO
    const fetchBusinesses = async () => {
        if (!userData || (userData.role !== 'BusinessOwner' && userData.role !== 'Admin')) return;
        
        try {
            const token = localStorage.getItem('auth_token');
            const response = await fetch('http://localhost:8080/api/businesses/my-businesses', {
                headers: {
                    'Authorization': `Bearer ${token}`,
                    'Content-Type': 'application/json'
                }
            });

            if (response.ok) {
                const data = await response.json();
                console.log('📥 Respuesta de negocios:', data);
                
                // ✅ MANEJO SEGURO: Asegurarse de que sea array
                if (data && data.businesses && Array.isArray(data.businesses)) {
                    setBusinesses(data.businesses);
                } else if (Array.isArray(data)) {
                    setBusinesses(data);
                } else {
                    console.warn('Respuesta de negocios no es array:', data);
                    setBusinesses([]); // Default a array vacío
                }
                console.log('✅ Negocios cargados correctamente');
            } else {
                console.error('Error cargando negocios:', response.status);
                setError('Error al cargar negocios');
                setBusinesses([]); // ✅ Asegurar array vacío en error
            }
        } catch (error) {
            console.error('Error cargando negocios:', error);
            setError('Error al cargar negocios');
            setBusinesses([]); // ✅ Asegurar array vacío en error
        }
    };

    // ✅ FUNCIÓN PARA CARGAR BOOKINGS CON MANEJO SEGURO
    const fetchBookings = async () => {
        try {
            const token = localStorage.getItem('auth_token');
            const response = await fetch('http://localhost:8080/api/bookings/my-bookings', {
                headers: {
                    'Authorization': `Bearer ${token}`,
                    'Content-Type': 'application/json'
                }
            });

            if (response.ok) {
                const data = await response.json();
                console.log('📥 Respuesta de bookings:', data);
                
                // ✅ MANEJO SEGURO: Asegurarse de que sea array
                if (data && data.bookings && Array.isArray(data.bookings)) {
                    setBookings(data.bookings);
                } else if (Array.isArray(data)) {
                    setBookings(data);
                } else {
                    console.warn('Respuesta de bookings no es array:', data);
                    setBookings([]); // Default a array vacío
                }
                console.log('✅ Bookings cargados correctamente');
            } else {
                console.error('Error cargando bookings:', response.status);
                setBookings([]); // ✅ Asegurar array vacío en error
            }
        } catch (error) {
            console.error('Error cargando bookings:', error);
            setBookings([]); // ✅ Asegurar array vacío en error
        }
    };

    // ✅ EFECTO PARA CARGAR DATOS
    useEffect(() => {
        if (userData && isLoggedIn) {
            const loadData = async () => {
                setLoading(true);
                setError(null);
                
                await Promise.all([
                    fetchHotels(),
                    fetchBusinesses(),
                    fetchBookings()
                ]);
                
                setLoading(false);
            };
            
            loadData();
        }
    }, [userData, isLoggedIn]);

    // ✅ RETURNS CONDICIONALES DESPUÉS DE TODOS LOS HOOKS
    if (!isLoggedIn || !userData) {
        return <Navigate to="/" replace />;
    }

    const isHotelOwner = userData.role === 'HotelOwner';
    const isBusinessOwner = userData.role === 'BusinessOwner';
    const canAccessPortal = isHotelOwner || isBusinessOwner || userData.role === 'Admin';

    if (!canAccessPortal) {
        return <Navigate to="/" replace />;
    }

    if (loading) {
        return (
            <div className="min-h-screen bg-gray-100 flex justify-center items-center">
                <div className="bg-white p-8 rounded-lg shadow">
                    <div className="text-xl">Cargando portal...</div>
                </div>
            </div>
        );
    }

    // ✅ DETERMINAR TABS SEGÚN EL ROL
    const getAvailableTabs = () => {
        const tabs = [{ id: 'overview', name: 'Resumen', icon: '📊' }];
        
        if (isHotelOwner || userData.role === 'Admin') {
            tabs.push({ id: 'hotels', name: 'Mis Hoteles', icon: '🏨' });
        }
        
        if (isBusinessOwner || userData.role === 'Admin') {
            tabs.push({ id: 'businesses', name: 'Mis Restaurantes', icon: '🍽️' });
        }
        
        tabs.push({ id: 'bookings', name: 'Reservas', icon: '📅' });
        
        return tabs;
    };

    return (
        <div className="min-h-screen bg-gray-100">
            {/* Header */}
            <div className="bg-white shadow">
                <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div className="flex justify-between items-center py-6">
                        <div>
                            <h1 className="text-2xl lg:text-3xl font-bold text-gray-900">
                                Portal de {isHotelOwner ? 'Hoteles' : isBusinessOwner ? 'Restaurantes' : 'Administración'}
                            </h1>
                            <p className="text-gray-600">
                                Bienvenido, {userData.first_name} {userData.last_name}
                            </p>
                        </div>
                        <Link to="/" className="text-blue-600 hover:text-blue-800">
                            ← Volver al inicio
                        </Link>
                    </div>

                    {/* Tabs */}
                    <div className="border-b border-gray-200">
                        {getAvailableTabs().map((tab) => (
                            <button
                                key={tab.id}
                                onClick={() => setActiveTab(tab.id)}
                                className={`py-2 px-4 border-b-2 font-medium text-sm ${
                                    activeTab === tab.id
                                        ? 'border-blue-500 text-blue-600'
                                        : 'border-transparent text-gray-500 hover:text-gray-700'
                                }`}
                            >
                                {tab.icon} {tab.name}
                            </button>
                        ))}
                    </div>
                </div>
            </div>

            {/* Content */}
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                {error && (
                    <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-6">
                        {error}
                    </div>
                )}

                {activeTab === 'overview' && (
                    <OverviewTab 
                        hotels={hotels} 
                        businesses={businesses} 
                        bookings={bookings} 
                        userRole={userData.role}
                    />
                )}
                {activeTab === 'hotels' && <HotelsTab hotels={hotels} />}
                {activeTab === 'businesses' && <BusinessesTab businesses={businesses} />}
                {activeTab === 'bookings' && <BookingsTab bookings={bookings} />}
            </div>
        </div>
    );
}

// ✅ COMPONENTE OVERVIEW CON VALIDACIÓN DE ARRAYS
function OverviewTab({ hotels, businesses, bookings, userRole }) {
    // ✅ ASEGURAR QUE TODOS SON ARRAYS ANTES DE USAR .filter()
    const safeHotels = Array.isArray(hotels) ? hotels : [];
    const safeBusinesses = Array.isArray(businesses) ? businesses : [];
    const safeBookings = Array.isArray(bookings) ? bookings : [];

    const totalHotels = safeHotels.length;
    const totalBusinesses = safeBusinesses.length;
    const totalBookings = safeBookings.length;
    const pendingHotels = safeHotels.filter(h => h.status === 'pending').length;
    const pendingBusinesses = safeBusinesses.filter(b => b.status === 'pending').length;

    return (
        <div className="space-y-6">
            {/* Métricas */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                {(userRole === 'HotelOwner' || userRole === 'Admin') && (
                    <>
                        <MetricCard title="Total Hoteles" value={totalHotels} color="blue" />
                        <MetricCard title="Hoteles Pendientes" value={pendingHotels} color="yellow" />
                    </>
                )}
                
                {(userRole === 'BusinessOwner' || userRole === 'Admin') && (
                    <>
                        <MetricCard title="Total Restaurantes" value={totalBusinesses} color="red" />
                        <MetricCard title="Restaurantes Pendientes" value={pendingBusinesses} color="yellow" />
                    </>
                )}
                
                <MetricCard title="Total Reservas" value={totalBookings} color="green" />
            </div>

            {/* Actividad Reciente */}
            <div className="bg-white rounded-lg shadow p-6">
                <h3 className="text-lg font-semibold mb-4">Actividad Reciente</h3>
                
                {totalHotels === 0 && totalBusinesses === 0 && totalBookings === 0 ? (
                    <div className="text-center py-8">
                        <p className="text-gray-500 mb-4">No tienes actividad reciente</p>
                        <div className="space-x-4">
                            {(userRole === 'HotelOwner' || userRole === 'Admin') && (
                                <Link
                                    to="/portal/nuevo-hotel"
                                    className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
                                >
                                    Registrar Hotel
                                </Link>
                            )}
                            {(userRole === 'BusinessOwner' || userRole === 'Admin') && (
                                <Link
                                    to="/registrar-restaurante"
                                    className="bg-red-600 text-white px-4 py-2 rounded hover:bg-red-700"
                                >
                                    Registrar Restaurante
                                </Link>
                            )}
                        </div>
                    </div>
                ) : (
                    <div className="space-y-4">
                        {/* Mostrar hoteles recientes */}
                        {safeHotels.slice(0, 3).map((hotel) => (
                            <div key={hotel.id} className="flex items-center justify-between border-b pb-4">
                                <div>
                                    <h4 className="font-medium">{hotel.name}</h4>
                                    <p className="text-sm text-gray-500">{hotel.location}</p>
                                </div>
                                <StatusBadge status={hotel.status} />
                            </div>
                        ))}
                        
                        {/* Mostrar negocios recientes */}
                        {safeBusinesses.slice(0, 3).map((business) => (
                            <div key={business.id} className="flex items-center justify-between border-b pb-4">
                                <div>
                                    <h4 className="font-medium">{business.name}</h4>
                                    <p className="text-sm text-gray-500">{business.location}</p>
                                </div>
                                <StatusBadge status={business.status} />
                            </div>
                        ))}
                    </div>
                )}
            </div>
        </div>
    );
}

// ✅ COMPONENTE HOTELTAB CON VALIDACIÓN
function HotelsTab({ hotels }) {
    const safeHotels = Array.isArray(hotels) ? hotels : [];
    
    if (safeHotels.length === 0) {
        return (
            <div className="bg-white rounded-lg shadow p-8 text-center">
                <h3 className="text-xl font-semibold mb-4">No tienes hoteles registrados</h3>
                <p className="text-gray-600 mb-6">Registra tu primer hotel para comenzar</p>
                <Link
                    to="/portal/nuevo-hotel"
                    className="bg-blue-600 text-white px-6 py-3 rounded-lg hover:bg-blue-700 transition-colors"
                >
                    + Registrar Hotel
                </Link>
            </div>
        );
    }

    return (
        <div className="space-y-6">
            <div className="flex justify-between items-center">
                <h2 className="text-2xl font-bold">Mis Hoteles</h2>
                <Link
                    to="/portal/nuevo-hotel"
                    className="bg-blue-600 text-white px-6 py-3 rounded-lg hover:bg-blue-700 transition-colors"
                >
                    + Nuevo Hotel
                </Link>
            </div>

            <div className="grid gap-6">
                {safeHotels.map((hotel) => (
                    <div key={hotel.id} className="bg-white rounded-lg shadow p-6">
                        <div className="flex justify-between items-start">
                            <div className="flex-1">
                                <h3 className="text-xl font-semibold mb-2">{hotel.name}</h3>
                                <p className="text-gray-600 mb-2">{hotel.description}</p>
                                <p className="text-sm text-gray-500">
                                    📍 {hotel.location} • {hotel.address}
                                </p>
                                <p className="text-lg font-bold text-green-600 mt-2">
                                    ${hotel.price} MXN/noche
                                </p>
                            </div>
                            
                            <div className="flex flex-col items-end space-y-2">
                                <StatusBadge status={hotel.status} />
                                <div className="flex space-x-2">
                                    <Link
                                        to={`/portal/editar-hotel/${hotel.id}`}
                                        className="text-blue-600 hover:text-blue-800 text-sm"
                                    >
                                        Editar
                                    </Link>
                                </div>
                            </div>
                        </div>
                        
                        {hotel.status === 'pending' && (
                            <div className="mt-4 p-4 bg-yellow-50 rounded-lg">
                                <p className="text-yellow-800 text-sm">
                                    ⏳ Tu hotel está pendiente de aprobación por el administrador
                                </p>
                            </div>
                        )}
                    </div>
                ))}
            </div>
        </div>
    );
}

// ✅ COMPONENTE BUSINESSESTAB CON VALIDACIÓN
function BusinessesTab({ businesses }) {
    const safeBusinesses = Array.isArray(businesses) ? businesses : [];
    
    if (safeBusinesses.length === 0) {
        return (
            <div className="bg-white rounded-lg shadow p-8 text-center">
                <h3 className="text-xl font-semibold mb-4">No tienes restaurantes registrados</h3>
                <p className="text-gray-600 mb-6">Registra tu primer restaurante para comenzar</p>
                <Link
                    to="/registrar-restaurante"
                    className="bg-red-600 text-white px-6 py-3 rounded-lg hover:bg-red-700 transition-colors"
                >
                    + Registrar Restaurante
                </Link>
            </div>
        );
    }

    return (
        <div className="space-y-6">
            <div className="flex justify-between items-center">
                <h2 className="text-2xl font-bold">Mis Restaurantes</h2>
                <Link
                    to="/registrar-restaurante"
                    className="bg-red-600 text-white px-6 py-3 rounded-lg hover:bg-red-700 transition-colors"
                >
                    + Nuevo Restaurante
                </Link>
            </div>

            <div className="grid gap-6">
                {safeBusinesses.map((business) => (
                    <div key={business.id} className="bg-white rounded-lg shadow p-6">
                        <div className="flex justify-between items-start">
                            <div className="flex-1">
                                <h3 className="text-xl font-semibold mb-2">{business.name}</h3>
                                <p className="text-gray-600 mb-2">{business.description}</p>
                                <p className="text-sm text-gray-500">
                                    📍 {business.location} • {business.address}
                                </p>
                            </div>
                            
                            <div className="flex flex-col items-end space-y-2">
                                <StatusBadge status={business.status} />
                            </div>
                        </div>
                        
                        {business.status === 'pending' && (
                            <div className="mt-4 p-4 bg-yellow-50 rounded-lg">
                                <p className="text-yellow-800 text-sm">
                                    ⏳ Tu restaurante está pendiente de aprobación por el administrador
                                </p>
                            </div>
                        )}
                    </div>
                ))}
            </div>
        </div>
    );
}

// ✅ COMPONENTE BOOKINGSTAB CON VALIDACIÓN
function BookingsTab({ bookings }) {
    const safeBookings = Array.isArray(bookings) ? bookings : [];
    
    if (safeBookings.length === 0) {
        return (
            <div className="bg-white rounded-lg shadow p-8 text-center">
                <h3 className="text-xl font-semibold mb-4">No tienes reservas</h3>
                <p className="text-gray-600">Las reservas de tus clientes aparecerán aquí</p>
            </div>
        );
    }

    return (
        <div className="space-y-6">
            <h2 className="text-2xl font-bold">Reservas de mis Servicios</h2>
            
            <div className="bg-white rounded-lg shadow overflow-hidden">
                <div className="overflow-x-auto">
                    <table className="min-w-full divide-y divide-gray-200">
                        <thead className="bg-gray-50">
                            <tr>
                                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                                    Cliente
                                </th>
                                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                                    Servicio
                                </th>
                                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                                    Fechas
                                </th>
                                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                                    Total
                                </th>
                                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                                    Estado
                                </th>
                            </tr>
                        </thead>
                        <tbody className="bg-white divide-y divide-gray-200">
                            {safeBookings.map((booking) => (
                                <tr key={booking.id}>
                                    <td className="px-6 py-4 whitespace-nowrap">
                                        <div className="text-sm font-medium text-gray-900">
                                            {booking.customer_name || 'Cliente'}
                                        </div>
                                        <div className="text-sm text-gray-500">
                                            {booking.customer_email}
                                        </div>
                                    </td>
                                    <td className="px-6 py-4 whitespace-nowrap">
                                        <div className="text-sm text-gray-900">{booking.hotel_name}</div>
                                        <div className="text-sm text-gray-500">{booking.hotel_location}</div>
                                    </td>
                                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                        {booking.check_in} - {booking.check_out}
                                    </td>
                                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                        ${booking.total_price}
                                    </td>
                                    <td className="px-6 py-4 whitespace-nowrap">
                                        <StatusBadge status={booking.status} />
                                    </td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    );
}

// ✅ COMPONENTES AUXILIARES
function MetricCard({ title, value, color }) {
    const colorClasses = {
        blue: 'bg-blue-50 text-blue-700',
        yellow: 'bg-yellow-50 text-yellow-700',
        red: 'bg-red-50 text-red-700',
        green: 'bg-green-50 text-green-700',
    };

    return (
        <div className={`${colorClasses[color]} p-6 rounded-lg`}>
            <h3 className="text-sm font-medium">{title}</h3>
            <p className="text-2xl font-bold mt-2">{value}</p>
        </div>
    );
}

function StatusBadge({ status }) {
    const statusConfig = {
        pending: { bg: 'bg-yellow-100', text: 'text-yellow-800', label: 'Pendiente' },
        approved: { bg: 'bg-green-100', text: 'text-green-800', label: 'Aprobado' },
        rejected: { bg: 'bg-red-100', text: 'text-red-800', label: 'Rechazado' },
    };

    const config = statusConfig[status] || statusConfig.pending;

    return (
        <span className={`${config.bg} ${config.text} px-2 py-1 rounded-full text-xs font-medium`}>
            {config.label}
        </span>
    );
}

export default PortalPage;