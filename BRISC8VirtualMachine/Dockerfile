FROM mcr.microsoft.com/dotnet/runtime:6.0 AS base
WORKDIR /app

FROM mcr.microsoft.com/dotnet/sdk:6.0 AS build
WORKDIR /src
COPY ["BRISC8VirtualMachine/BRISC8VirtualMachine.csproj", "BRISC8VirtualMachine/"]
RUN dotnet restore "BRISC8VirtualMachine/BRISC8VirtualMachine.csproj"
COPY . .
WORKDIR "/src/BRISC8VirtualMachine"
RUN dotnet build "BRISC8VirtualMachine.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish "BRISC8VirtualMachine.csproj" -c Release -o /app/publish

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "BRISC8VirtualMachine.dll"]
